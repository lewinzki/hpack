// Comments enclosed in quotes are citations from the HPACK draft:
// http://tools.ietf.org/html/draft-ietf-httpbis-header-compression-07
use std::str;
use collections::HashSet;

use header_table::{HeaderTable, DEFAULT_HEADER_TABLE_SIZE};
use header_set::HeaderSet;
use reference_set::ReferenceSet;
use integer_representation::decode_int;
use header_field::HeaderField;
use static_header_table::StaticHeaderTable;
use huffman::huffman_decoder::HuffmanDecoder;

// Macro rule to unwrap an option
// If None, the function using this macro will return with None
// Invoke it like `propagate_err!(option)`
macro_rules! propagate_err(
    ($inp:expr) => ( 
        match $inp {
            Some(x) => x,
            None    => return None,
        }
    );
)

/// An implementation of an HPACK decoding context for HTTP/2.
pub struct Decoder {
    priv header_table:        ~HeaderTable,
    priv reference_set:       ~ReferenceSet,
    priv header_set:          ~HeaderSet,
    priv static_header_table: ~StaticHeaderTable,
    priv huffman_decoder:     HuffmanDecoder,
}

impl Decoder {
    /// Create an empty decoding context.
    pub fn new() -> Decoder {
        Decoder {
            header_table:        ~HeaderTable::new(DEFAULT_HEADER_TABLE_SIZE),
            reference_set:       ~ReferenceSet::new(),
            header_set:          ~HeaderSet::new(),
            static_header_table: ~StaticHeaderTable::new(),
            huffman_decoder:     HuffmanDecoder::new(),
        }
    }

    /// Decode a headerblock into a set of header fields. Return `None` if a decoding error has occurred.
    pub fn decode(&mut self, mut header_block: ~[u8]) -> Option<~HashSet<HeaderField>> {
        // TODO: Should we just empty the existing instead (memory leak) ?
        self.header_set = ~HeaderSet::new();
        self.reference_set.reset();

        while header_block.len() > 0 {
            let representation_type = header_block[0];

            if representation_type >= 0x80 {                // 1XXX XXXX = Indexed Header Field
                header_block = propagate_err!(self.decode_indexed_header(header_block));

            } else if representation_type == 0x00 ||        // 0000 0000 = Literal Header Field - New Name
                      representation_type == 0x40 ||        // 0100 0000 = Literal Header Field - New Name
                      representation_type == 0x10 {         // 0001 0000 = Literal Header Field never Indexed - New Name
                header_block = propagate_err!(self.decode_string_literal(header_block));

            } else if representation_type & 0xC0 == 0x40 || // 01XX XXXX & 1100 0000 == 0100 0000
                      representation_type & 0xF0 == 0x00 || // 0000 XXXX & 1111 0000 == 0000 0000
                      representation_type & 0xF0 == 0x10 {  // 0001 XXXX & 1111 0000 == 0001 0000
                header_block = propagate_err!(self.decode_indexed_literal(header_block));

            } else if representation_type & 0xE0 == 0x20 {  // 001X XXXX & 1110 0000 == 0010 0000
                header_block = propagate_err!(self.decode_context_update(header_block));
            } else {
                return None;
            }
        }

        // 3.2.2.  Reference Set Emission
        //
        // Emit all header fields in the reference set that are not already emitted
        for (_, value) in self.reference_set.references.iter() {
            match value.clone() {
                (hf, false) => self.header_set.emit(hf.clone()),
                _ => {}
            } 
        }

        // Return the header fields in the header_set
        Some(self.header_set.get_header_fields())
    }

    fn decode_indexed_literal(&mut self, mut header_block: ~[u8]) -> Option<~[u8]> {
        let indexing = header_block[0] & 0x40 == 0x40;

        let index_size =
            if indexing {
                6
            } else {
                4
            };
        let (index, buffer) = propagate_err!(decode_int(header_block, index_size));
        let (value, buffer) = propagate_err!(self.read_string(buffer));
        header_block = buffer;

        let mut name;

        if index > self.header_table.len() {
            // Static header table index
            let static_index = index - self.header_table.len();
            let hf = propagate_err!(self.static_header_table.get(static_index));
            name = hf.key;
        } else {
            // Header table index
            let hf = propagate_err!(self.header_table.get(index));
            name = hf.key;
        }

        let updated_header_field = HeaderField::new(name, propagate_err!(str::from_utf8_owned(value)));

        if indexing {
            self.header_set.emit(updated_header_field.clone());
            self.header_table.add(updated_header_field.clone());
            self.reference_set.add(updated_header_field.clone(), true);

            self.evict();
        } else {
            self.header_set.emit(updated_header_field.clone());
        }

        Some(header_block)
    }

    fn decode_string_literal(&mut self, mut header_block: ~[u8]) -> Option<~[u8]> {
        let indexing = header_block.shift().unwrap() == 0x40; // Remove the first octet and set indexing to true if 0100 0000 

        let (name, buffer)  = propagate_err!(self.read_string(header_block));
        let (value, buffer) = propagate_err!(self.read_string(buffer));

        header_block = buffer;

        let hf = HeaderField::new(propagate_err!(str::from_utf8_owned(name)), propagate_err!(str::from_utf8_owned(value)));

        if indexing {
            self.header_set.emit(hf.clone());
            self.header_table.add(hf.clone());
            self.reference_set.add(hf.clone(), true);

            self.evict();
        } else {
            self.header_set.emit(hf.clone());
        }

        Some(header_block)
    }

    fn decode_indexed_header(&mut self, mut header_block: ~[u8]) -> Option<~[u8]> {
        let (index, buffer) = propagate_err!(decode_int(header_block, 7));
        header_block = buffer;

        if index > self.header_table.len() { // Look in static table
            let static_index = index - self.header_table.len();
            let hf = propagate_err!(self.static_header_table.get(static_index));

            if self.reference_set.has(&hf) {
                // "An _indexed representation_ corresponding to an entry _present_ in
                // the reference set entails the following actions:
                //
                // o  The entry is removed from the reference set."

                match self.reference_set.remove(&hf) {
                    false => return None,
                    true => {},
                }
            } else {
                // "*  The header field corresponding to the referenced entry is
                //     emitted.
                // 
                //  *  The referenced static entry is inserted at the beginning of the
                //     header table.
                // 
                //  *  A reference to this new header table entry is added to the
                //     reference set (except if this new entry didn't fit in the
                //     header table)."

                self.header_set.emit(hf.clone());
                self.header_table.add(hf.clone());

                // We add the header field to the reference_set no matter what
                // If the header field don't fit into the header_table we will
                // remove it from the reference_set during the eviction

                self.reference_set.add(hf.clone(), true);

                self.evict();
            }

        } else { // Look in header table
            let hf = propagate_err!(self.header_table.get(index));

            if self.reference_set.has(&hf) {
                // "An _indexed representation_ corresponding to an entry _present_ in
                // the reference set entails the following actions:
                //
                // o  The entry is removed from the reference set."

                match self.reference_set.remove(&hf) {
                    false => return None,
                    true => {},
                }
            } else {
                // "*  The header field corresponding to the referenced entry is
                //     emitted.
                //
                //  *  The referenced header table entry is added to the reference
                //     set."

                self.header_set.emit(hf.clone());
                self.reference_set.add(hf.clone(), true);
            }
        }

        Some(header_block)
    }

    fn decode_context_update(&mut self, header_block: ~[u8]) -> Option<~[u8]> {
        let (data, buffer) = propagate_err!(decode_int(header_block.clone(), 4));

        if header_block[0] & 0xF0 == 0x30 { // 0011 XXXX & 1111 0000 == 0011 0000
            if data == 0 {
                // Empty reference set
                self.reference_set.empty();
            }

            // Possibly other changes in the future of HPACK
        } else {
            // Change header table size
            // TODO: Check that the new size is lower than or equal to SETTINGS_HEADER_TABLE_SIZE 
            // http://tools.ietf.org/html/draft-ietf-httpbis-header-compression-07#section-4.4
            self.header_table.set_max_size(data);

            self.evict();
        }

        Some(buffer)
    }

    /*
     * Reads a string from the header block (and consumes it)
     */
    fn read_string(&mut self, mut header_block: ~[u8]) -> Option<(~[u8], ~[u8])> { 
        let mut string;        

        let huffman_encoded = header_block[0] & 0x80 == 0x80; // 1XXX XXXX & 1000 0000 == 1000 0000
        let (string_length, buffer) = propagate_err!(decode_int(header_block, 7));
        header_block = buffer;

        if huffman_encoded {
            let decoded_string_opt = self.huffman_decoder.decode(header_block.slice(0, string_length).to_owned());
            string = propagate_err!(decoded_string_opt);
        } else {
            string = header_block.slice(0, string_length).to_owned();
        }

        for _ in range(0, string_length) {
            header_block.shift();
        }

        Some((string, header_block))
    }

    // Evict header fields until the header table is wihtin its allowed size.
    // We remove the oldest header field (remember, we add from the front)
    // and also remove it from the reference set.
    fn evict(&mut self) {
        while self.header_table.size() > self.header_table.get_max_size() && self.header_table.len() > 0 {
            let header_table_length = self.header_table.len();
            // If this unwrap fails, something is rotten in Denmark
            // I.e. We assume it is always within bounds to remove the last element 
            let removed_header_field = self.header_table.remove(header_table_length).unwrap();
            self.reference_set.remove(&removed_header_field);
        }
    }
}

#[cfg(test)]
mod decode_test {
    use integer_representation::encode_int; 
    use decoder::Decoder;
    use header_field::HeaderField;


    #[test]
    fn decode_test() {
        let mut decoder = Decoder::new();
        
        let mut index = encode_int(5, 7);
        index[0] += 128; // Flip the first bit

        let frame0 = index; // Indexed header with index = 5

        let header_fields = decoder.decode(frame0).unwrap();
        let h0 = HeaderField::new(~":path", ~"/index.html");
        assert!(header_fields.contains(&h0));



        let h1 = HeaderField::new(~"foo", ~"bar");
        let name = ~"foo";
        let value = ~"bar";
        let name_length = encode_int(name.clone().len(), 7);
        let value_length = encode_int(value.clone().len(), 7);
        let mut frame1 = ~[0x40];
        frame1.push_all_move(name_length);
        frame1.push_all_move(name.into_bytes());
        frame1.push_all_move(value_length);
        frame1.push_all_move(value.into_bytes());

        let header_fields = decoder.decode(frame1).unwrap();

        assert!(header_fields.contains(&h0));
        assert!(header_fields.contains(&h1));



        let h2 = HeaderField::new(~"foo", ~"baz");
        let index = encode_int(1, 6);
        let value = ~"baz";
        let value_length = encode_int(value.clone().len(), 7);
        let mut frame2: ~[u8] = ~[];
        frame2.push_all_move(~[0x81]);
        frame2.push_all_move(index);
        frame2.push_all_move(value_length);
        frame2.push_all_move(value.into_bytes());

        let header_fields = decoder.decode(frame2).unwrap();

        assert!(header_fields.contains(&h0));
        assert!(header_fields.contains(&h2));
        assert!(!header_fields.contains(&h1));
    }
}