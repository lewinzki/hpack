use collections::HashMap;

#[deriving(Clone)]
pub struct HeaderCollection {
    pub header_fields: HashMap<~str, ~str>,
}

impl HeaderCollection {
    pub fn new() -> HeaderCollection {
        HeaderCollection {
            header_fields: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: ~str, value: ~str) {
        self.header_fields.insert_or_update_with(key, value.clone(), |_, v| *v = *v + "\0" + value.clone());
    }

    pub fn get(&self, key: &~str) -> ~[~str] {
        let mut values = ~[];

        match self.header_fields.find(key) {
            Some(value) => {
                for val in value.split('\0') {
                    values.push(val.clone().to_owned());                    
                }
            },
            None => {}
        }

        values
    }

    pub fn merge(&mut self, other: HeaderCollection) {
        for (key, value) in other.header_fields.iter() {
            self.add(key.to_owned(), value.to_owned());
        }
    }
}