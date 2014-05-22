// TODO: Being able to resize the header table (API feature)

use collections::HashSet;

use header_table::{HeaderTable, DEFAULT_HEADER_TABLE_SIZE};
use reference_set::ReferenceSet;
use header_field::HeaderField;
use static_header_table::StaticHeaderTable;
use representation::{IndexedHeader, IndexedLiteral, NamedLiteral, ContextUpdate, Representation};
use huffman::huffman_encoder::HuffmanEncoder;

/// An implementation of an HPACK encoding context for HTTP/2.
pub struct Encoder {
    priv header_table:        ~HeaderTable,
    priv reference_set:       ~ReferenceSet,
    priv static_header_table: ~StaticHeaderTable,
    priv huffman_encoder:     HuffmanEncoder,
}

impl Encoder {
    /// Create an empty encoding context.
    pub fn new() -> Encoder {
        Encoder {
            header_table:        ~HeaderTable::new(DEFAULT_HEADER_TABLE_SIZE),
            reference_set:       ~ReferenceSet::new(),
            static_header_table: ~StaticHeaderTable::new(),
            huffman_encoder:     HuffmanEncoder::new(),
        }
    }

    /// Return a headerblock of encoding a given set of header fields with the current context.
    pub fn encode(&mut self, fields: ~HashSet<HeaderField>) -> ~[u8] {
        let mut ref_set: HashSet<HeaderField> = HashSet::new();
        let mut header_block: ~[u8] = ~[];

        for (hf, _) in self.reference_set.references.iter() {
            ref_set.insert(hf.clone());
        }

        let mut to_add = fields.difference(&ref_set);  // Creates an iterator containing the difference
        let mut to_remove = ref_set.difference(fields);

        // If the number of fields to remove is larger than half the length
        // of the reference_set => Empty the reference set
        // TODO: Make it a setting instead of statically use 50%
        if to_remove.len() > self.reference_set.len() / 2 {
            // Create an context update of [0011 0000]
            let ref_set_empty = ContextUpdate::new(true, 0);
            header_block = ref_set_empty.encode();
        } else {
            // Create indexed headers to all the references we want removed
            // from the reference_set
            for hf in to_remove {
                match self.header_table.find(hf.clone()) {
                    Some((index, true)) => {
                        let indexed_header = IndexedHeader::new(index);
                        header_block.push_all_move(indexed_header.encode());

                        self.reference_set.remove(hf);
                    },
                    Some((_, false)) => { fail!("Tried to remove a header field with a different value.") },
                    None => { fail!("Tried to remove header field not present in the header nor the static header table.") } 
                }
            }
        }

        for hf in to_add {
                match self.find_header(hf.clone()) {
                    // (Index, PerfectMatch)
                    Some((index, true)) => {
                        let indexed_header = IndexedHeader::new(index);
                        header_block.push_all_move(indexed_header.encode());

                        self.reference_set.add(hf.clone(), true);

                        // If index is in the static header table => add to the header table
                        if index > self.header_table.len() {
                            self.header_table.add(hf.clone());
                            self.evict();
                        }
                    },
                    Some((index, false)) => {
                        // Create an indexed literal without indexing.
                        // We do this since the benefit from indexing the header field
                        // might not be that big - the chance of the next request wanting to use
                        // the same header value is little (this is an assumption)
                        // Room for optimization: e.g. Index if the header key is "server"

                        let mut value_bytes;

                        let huffman_value = self.huffman_encoder.encode(hf.value.clone().into_bytes());
                        let value_use_huffman = huffman_value.len() < hf.value.len();

                        if value_use_huffman {
                            value_bytes = huffman_value;
                        } else {
                            value_bytes = hf.value.clone().into_bytes();
                        }

                        let indexed_literal = IndexedLiteral::new(false, false, index, value_use_huffman, value_bytes);
                        header_block.push_all_move(indexed_literal.encode());
                    },
                    None => { 
                        // Not in any of the tables. Send as Named Literal and add to header table and reference set

                        let mut key_bytes;
                        let mut value_bytes;

                        let huffman_key = self.huffman_encoder.encode(hf.key.clone().into_bytes());
                        let huffman_value = self.huffman_encoder.encode(hf.value.clone().into_bytes());
                        let key_use_huffman = huffman_key.len() < hf.key.len();
                        let value_use_huffman = huffman_value.len() < hf.value.len();

                        if key_use_huffman {
                            key_bytes = huffman_key;
                        } else {
                            key_bytes = hf.key.clone().into_bytes();
                        }

                        if value_use_huffman {
                            value_bytes = huffman_value;
                        } else {
                            value_bytes = hf.value.clone().into_bytes();
                        }

                        let named_literal = NamedLiteral::new(true, false, key_use_huffman, key_bytes, value_use_huffman, value_bytes);
                        header_block.push_all_move(named_literal.encode());

                        self.header_table.add(hf.clone());
                        self.reference_set.add(hf.clone(), true);

                        self.evict();
                    } 
                }
            }

        header_block
    }

    // Search for a header field in the header table and the static header table. 
    // If not found, returns (0, false) - a valid index is > 0.
    // If found, returns the index and wether or not the value did also match.
    fn find_header(&self, hf: HeaderField) -> Option<(uint, bool)> {
        match self.header_table.find(hf.clone()) {
            Some(x) => return Some(x),
            None    => {
                match self.static_header_table.find(hf.clone()) {
                    Some((i, p)) => return Some((i + self.header_table.len(), p)),
                    None => return None
                }
            }
        }
    }

    // Evict header fields until the header table is within its allowed size.
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

// #[cfg(test)]
// mod encoder_test {
//     use integer_representation::encode_int;
//     use encoder::Encoder;
//     use header_field::HeaderField;
//     use collections::HashSet;


//     #[test]
//     fn encoder_test() {
//         let mut encoder = Encoder::new();

//         let mut hs0: HashSet<HeaderField> = HashSet::new();
//         let h0 = HeaderField::new(~"fooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooo", ~"bar");
//         hs0.insert(h0.clone());

//         let mut hb0 = encoder.encode(~hs0);

//         let h0_name_len = h0.clone().key.len();
//         let h0_value_len = h0.clone().value.len();

//         let h0_name_len_enc = encode_int(h0_name_len, 7);
//         let h0_value_len_enc = encode_int(h0_value_len, 7);    

//         assert!(hb0[0] == 0x00);
//         hb0.shift();
//         assert!(hb0.slice(0, h0_name_len_enc.len()) == h0_name_len_enc);
//         for _ in range(0, h0_name_len_enc.len()) {
//             hb0.shift();
//         }

//         assert!(hb0.slice(0, h0_name_len) == (~"fooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooofooFOOfooo").into_bytes());
//         for _ in range(0, h0_name_len) {
//             hb0.shift();
//         }    
//         assert!(hb0.slice(0, h0_value_len_enc.len()) == h0_value_len_enc);
//         for _ in range(0, h0_value_len_enc.len()) {
//             hb0.shift();
//         }
//         assert!(hb0.slice(0, h0_value_len) == (~"bar").into_bytes());
//         assert!(hb0.len() == h0_value_len);
//     }
// }