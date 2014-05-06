use collections::HashSet;

use header_field::HeaderField;

pub struct HeaderSet {
    fields: ~HashSet<HeaderField>
}

impl HeaderSet {
    pub fn new() -> HeaderSet {
        HeaderSet {
            fields: ~HashSet::new()
        }
    }

    pub fn emit(&mut self, field: HeaderField) {
        self.fields.insert(field);
    }

    // Used for testing
    #[allow(dead_code)]
    pub fn len(&self) -> uint {
        self.fields.len()
    }

    pub fn get_header_fields(&self) -> ~HashSet<HeaderField> {
        self.fields.clone()
    }
}

#[test]
fn header_set_test() {
    let mut hs = HeaderSet::new();
    let h0 = HeaderField::new(~"foo", ~"bar");
    hs.emit(h0);

    assert!(hs.len() == 1);

    let h1 = HeaderField::new(~"foo1", ~"bar1");
    hs.emit(h1);

    assert!(hs.len() == 2);
    let h2 = HeaderField::new(~"foo2", ~"bar2");
    hs.emit(h2);

    assert!(hs.len() == 3);
}