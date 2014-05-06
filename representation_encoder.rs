use representation::{Representation, IndexedHeader, IndexedLiteral, NamedLiteral, ContextUpdate};
use integer_representation::{encode_int};

impl Representation for IndexedHeader {
    fn encode(&self) -> ~[u8] {
        let mut buffer: ~[u8] = encode_int(self.index, 7);
        buffer[0] = buffer[0] | 0x80; // We set the top bit

        buffer
    }
}

impl Representation for IndexedLiteral {
    fn encode(&self) -> ~[u8] {
        let mask: u8;
        let mut buffer: ~[u8];

        // If never indexed is true we don't care about indexing
        if self.indexing && !self.never_indexed {          // | 0 | 0 | 0 | 0 |  Index (4+)   |
            buffer = encode_int(self.index, 4);

            mask = 0x00;
        } else if !self.indexing && !self.never_indexed {  // | 0 | 1 |      Index (6+)       |
            buffer = encode_int(self.index, 6);

            // Flip the second bit of the first byte/octet if indexing is set to false
            mask = 0x40;
        } else {                                           // | 0 | 0 | 0 | 1 |  Index (4+)   |
            buffer = encode_int(self.index, 4);

            // Never indexed
            // Flip the fourth bit
            mask = 0x10;
        }


        buffer[0] = buffer[0] | mask;

        if self.value_huffman {
            // TODO: Huffman
            fail!("Huffman not implemented");
        } else {
            let value_length = encode_int(self.value_length, 7);
            buffer.push_all_move(value_length);
            buffer.push_all_move(self.value_string.clone());
        }

        buffer
    }
}

impl Representation for NamedLiteral {
    fn encode(&self) -> ~[u8] {
        let mut buffer: ~[u8] = ~[];

        // If never indexed is true we don't care about indexing
        if self.indexing && !self.never_indexed {
            buffer.push(0x00); // 0000 0000
        } else if !self.indexing && !self.never_indexed {
            buffer.push(0x40); // 0100 0000
        } else {
            // Never indexed
            buffer.push(0x10); // 0001 0000
        }

        if self.name_huffman {
            // TODO: Huffman
            fail!("Huffman not implemented");
        } else {
            let name_length = encode_int(self.name_length, 7);
            buffer.push_all_move(name_length);
            buffer.push_all_move(self.name_string.clone());
        }

        if self.value_huffman {
            // TODO: Huffman
            fail!("Huffman not implemented");
        } else {
            let value_length = encode_int(self.value_length, 7);
            buffer.push_all_move(value_length);
            buffer.push_all_move(self.value_string.clone());
        }

        buffer
    }
}

impl Representation for ContextUpdate {
    fn encode(&self) -> ~[u8] {
        let mut buffer: ~[u8] = encode_int(self.data, 4);;
        let mask: u8;

        if self.flag {
            mask = 0x30;
        } else {
            mask = 0x20;
        }
        buffer[0] = buffer[0] | mask; // We set the top bit

        buffer
    }
}

#[test]
fn indexed_header_test() {
    let h0 = IndexedHeader::new(127);
    assert!(h0.encode()[0] == 255);


    let h1 = IndexedHeader::new(128);
    assert!(h1.encode()[0] == 255);
    assert!(h1.encode()[1] == 1);
}

#[test]
fn indexed_literal_test() {
    let s0 = (~"Hello").into_bytes();
    let h0 = IndexedLiteral::new(true, false, 14, false, s0.clone());
    assert!(h0.encode()[0] == 14);  // Is the index encoded correctly
    assert!(h0.encode()[1] == 5);   // The length of "Hello" in octets
    assert!(h0.encode()[2] == 72);  // The 1st character is 'H'
    assert!(h0.encode()[3] == 101); // The 2nd character is 'e'
    assert!(h0.encode()[4] == 108); // The 3rd character is 'l'
    assert!(h0.encode()[5] == 108); // The 4th character is 'l'
    assert!(h0.encode()[6] == 111); // The 5th character is 'o'

    let h1 = IndexedLiteral::new(false, false, 62, false, s0.clone());
    assert!(h1.encode()[0] == 126); // Is the index encoded correctly
    assert!(h1.encode()[1] == 5);   // The length of "Hello" in octets
    assert!(h1.encode()[2] == 72);  // The 1st character is 'H'
    assert!(h1.encode()[3] == 101); // The 2nd character is 'e'
    assert!(h1.encode()[4] == 108); // The 3rd character is 'l'
    assert!(h1.encode()[5] == 108); // The 4th character is 'l'
    assert!(h1.encode()[6] == 111); // The 5th character is 'o'
}

#[test]
fn named_literal_test() {
    let n0 = (~"Hello").into_bytes();
    let v0 = (~"World").into_bytes();
    let h0 = NamedLiteral::new(true, false, false, n0.clone(), false, v0.clone());
    assert!(h0.encode()[0] == 0);    // Is indexing set to true
    assert!(h0.encode()[1] == 5);    // The length of "Hello" in octets
    assert!(h0.encode()[2] == 72);   // The 1st character is 'H'
    assert!(h0.encode()[3] == 101);  // The 2nd character is 'e'
    assert!(h0.encode()[4] == 108);  // The 3rd character is 'l'
    assert!(h0.encode()[5] == 108);  // The 4th character is 'l'
    assert!(h0.encode()[6] == 111);  // The 5th character is 'o'
    assert!(h0.encode()[7] == 5);    // The length of "World" in octets
    assert!(h0.encode()[8] == 87);   // The 1st character is 'W'
    assert!(h0.encode()[9] == 111);  // The 2nd character is 'o'
    assert!(h0.encode()[10] == 114); // The 3rd character is 'r'
    assert!(h0.encode()[11] == 108); // The 4th character is 'l'
    assert!(h0.encode()[12] == 100); // The 5th character is 'd'

    let h1 = NamedLiteral::new(false, false, false, n0.clone(), false, v0.clone());
    assert!(h1.encode()[0] == 64);   // Is indexing set to false
    assert!(h1.encode()[1] == 5);    // The length of "Hello" in octets
    assert!(h1.encode()[2] == 72);   // The 1st character is 'H'
    assert!(h1.encode()[3] == 101);  // The 2nd character is 'e'
    assert!(h1.encode()[4] == 108);  // The 3rd character is 'l'
    assert!(h1.encode()[5] == 108);  // The 4th character is 'l'
    assert!(h1.encode()[6] == 111);  // The 5th character is 'o'
    assert!(h1.encode()[7] == 5);    // The length of "World" in octets
    assert!(h1.encode()[8] == 87);   // The 1st character is 'W'
    assert!(h1.encode()[9] == 111);  // The 2nd character is 'o'
    assert!(h1.encode()[10] == 114); // The 3rd character is 'r'
    assert!(h1.encode()[11] == 108); // The 4th character is 'l'
    assert!(h1.encode()[12] == 100); // The 5th character is 'd'
}