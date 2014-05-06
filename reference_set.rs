use collections::HashMap;

use header_field::HeaderField;

pub struct ReferenceSet {
    // <Key of headerfield, (headerfield, emitted)>
    // Key is inserted into the key of the hashmap to be able to compare only by keys.
    // (host:server1 and host:server2 should be the same header fields in the reference set but with different values)
    references: ~HashMap<~str, (HeaderField, bool)>
}

impl ReferenceSet {
    pub fn new() -> ReferenceSet {
        ReferenceSet {
            references: ~HashMap::new()
        }
    }

    // Number of reference fields in the set
    pub fn len(&self) -> uint {
        self.references.len()
    }

    pub fn add(&mut self, field: HeaderField, emitted: bool) {
        // TODO: Avoid using all them clones!
        self.references.insert_or_update_with(field.clone().key, (field.clone(), emitted), |_, v| *v = (field.clone(), emitted));
    }

    pub fn empty(&mut self) {
        self.references.clear();
    }

    // Is a given header field present in the reference set
    pub fn has(&self, field: &HeaderField) -> bool {
        self.references.contains_key(&field.key)
    }

    // Checks if a given header field exists in the reference set.
    // If it does    -> remove the reference and return true
    // If it doesn't -> return false 
    pub fn remove(&mut self, field: &HeaderField) -> bool {
        self.references.remove(&field.key)
    }

    // Set all references to "not emitted"
    pub fn reset(&mut self) {
        // Note: Possibly suboptimal
        let mut s = self.references.clone();
        for (_, value) in s.mut_iter() {
            match value.clone() {
                (hf, _) => self.add(hf, false)
            }            
        }
    }
}

#[test]
fn reference_set_test() {
    let mut rs = ReferenceSet::new();

    let h0 = &HeaderField::new(~"foo", ~"bar0");
    let h1 = &HeaderField::new(~"foo1", ~"bar00");
    let h2 = &HeaderField::new(~"foo2", ~"bar000");

    rs.add(h0.clone(), true);
    rs.add(h1.clone(), false);
    rs.add(h2.clone(), true);

    assert!(rs.len() == 3);

    assert!(rs.has(h0));
    assert!(rs.has(h1));
    assert!(rs.has(h2));


    let h3 = &HeaderField::new(~"foo2", ~"bar0000"); // Update f002
    rs.add(h3.clone(), true);

    assert!(rs.len() == 3);

    assert!(rs.has(h0));
    assert!(rs.has(h1));
    assert!(rs.has(h3));
    assert!(rs.has(h2));

    // Check that the values are correct.
    // Especially that "foo2" is "bar0000" and NOT "bar000"
    for (key, value) in rs.references.iter() {
        let (hf, _) = value.clone();

        if *key == ~"foo" {
            assert!(hf.value == ~"bar0");
        } else if *key == ~"foo1" {
            assert!(hf.value == ~"bar00");
        } else if *key == ~"foo2" {
            assert!(hf.value == ~"bar0000"); 
        } else {
            assert!(false); // Should fail if key is not one of the header fields' key
        }
    }



    rs.remove(h0);

    assert!(rs.len() == 2);

    assert!(!rs.has(h0));
    assert!(rs.has(h1));
    assert!(rs.has(h2));
    assert!(rs.has(h3));

    rs.reset();

    for (_, value) in rs.references.iter() {
        match value.clone() {
            (_, b) => assert!(b == false)
        }
    }



    rs.empty();

    assert!(rs.len() == 0);

    assert!(rs.has(h0) == false);
    assert!(rs.has(h1) == false);
    assert!(rs.has(h2) == false);
    assert!(rs.has(h3) == false);
}