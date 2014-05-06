use header_field::HeaderField;

pub struct HeaderTable {
    priv fields: ~[HeaderField],
    priv max_size: uint
}

pub static DEFAULT_HEADER_TABLE_SIZE: uint = 4096; 

impl HeaderTable {
    pub fn new(max_size: uint) -> HeaderTable {
        HeaderTable {
            fields: ~[],
            max_size: max_size
        }
    }

    pub fn set_max_size(&mut self, new_max_size: uint) {
        self.max_size = new_max_size;
    }

    pub fn get_max_size(&self) -> uint {
        self.max_size
    }


    // Search for a header field.
    // Return None if not found.
    // Return Some(index, full_match) where full_match is true
    // if both the name _and_ value match.
    // If only a partial match is found it will return the last 
    // partial match found.
    pub fn find(&self, hf: HeaderField) -> Option<(uint, bool)> {
        let mut partial_match = None; 

        for i in range(0, self.fields.len()) {
            if self.fields[i].key == hf.key {
                if self.fields[i].value == hf.value {
                    return Some((i + 1, true));
                } else {
                    partial_match = Some((i + 1, false));
                }
            }
        }

        partial_match
    }

    // The size of the header table in octets
    // i.e. the sum of header field's sizes
    pub fn size(&self) -> uint {
        self.fields.iter().fold(0, |r, f| r + f.size())
    }

    // Number of header fields in the table
    pub fn len(&self) -> uint {
        self.fields.len()
    }

    // Prepend a header field to the table
    pub fn add(&mut self, field: HeaderField) {
        self.fields.unshift(field);
    }

    // Return the header field at 'index' - 1
    // Returns None if out of bounds
    // Remeber, HPACK uses 1-indexing!!!
    pub fn get(&mut self, index: uint) -> Option<HeaderField> {
        if index < 1 || index > self.fields.len() {
            return None;
        }

        Some(self.fields[index - 1].clone())
    }

    // Remove and return a header field at an index
    // and shift all elements after the index one to the left
    pub fn remove(&mut self, index: uint) -> Option<HeaderField> {
        // Remember, HPACK uses 1-indexing!!!
        self.fields.remove(index - 1)
    }
}

#[test]
fn header_table_test() {
    let h0 = HeaderField::new(~"foo", ~"bar0");
    let h1 = HeaderField::new(~"foo1", ~"bar00");
    let h2 = HeaderField::new(~"foo2", ~"bar000");

    let s0 = h0.size();
    let s1 = h1.size();
    let s2 = h2.size();

    // max_size does not play a role in this test 
    let mut ht = ~HeaderTable { fields: ~[], max_size: 0 };

    ht.add(h0);
    ht.add(h1);
    ht.add(h2);

    assert!(ht.len() == 3);
    assert!(ht.size() == s0 + s1 + s2);

    // Check the order of the header fields
    assert!(ht.fields[0].key == ~"foo2");
    assert!(ht.fields[1].key == ~"foo1");
    assert!(ht.fields[2].key == ~"foo");

    ht.remove(2); // Remove the _second_ element.

    // Check that the second header field is removed
    // and that the last one has been shifted to the left
    assert!(ht.len() == 2);
    assert!(ht.size() == s0 + s2);

    assert!(ht.fields[0].key == ~"foo2");
    assert!(ht.fields[1].key == ~"foo");
}