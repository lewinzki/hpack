/// Struct representing an HTTP/2 header field.
#[deriving(Eq, TotalEq, Hash, Clone)]
pub struct HeaderField {
    key: ~str,
    value: ~str
}

impl HeaderField {
    /// Create a header field.
    pub fn new(key: ~str, value: ~str) -> HeaderField {
        HeaderField {
            key: key,
            value: value,
        }
    }

    /// Returns the size of the header field.
    /// The size of a header field is the sum of its name's length in octets (bytes) plus its value's length in octets plus 32.
    /// The 32 octets accounts for structure overhead (two pointers).        
    pub fn size(&self) -> uint {
        let key_size = self.key.len();
        let value_size = self.value.len();
        key_size + value_size + 32
    }
}