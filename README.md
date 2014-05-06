# Rust HPACK library for HTTP/2
This is a header compression library for HTTP/2 (HPACK).  
The library is developed under the specifications of the [HPACK draft-07](https://tools.ietf.org/html/draft-ietf-httpbis-header-compression-07).  
The implementation is not entirely complete. See the *Missing features* section, to see what is missing to fulfill the draft.


## Usage
```rust
  extern crate collections;
  extern crate hpack;

  use collections::HashSet;
  use hpack::{HeaderField, Encoder, Decoder};
  
  fn main() {
      // Create a header set
      let mut header_set: ~HashSet<HeaderField> = ~HashSet::new();

      // Create two header fields
      let hf1: HeaderField = HeaderField::new(~":status", ~"200");
      let hf2: HeaderField = HeaderField::new(~"foo", ~"bar"); 

      // Insert the two header fields into the header set
      header_set.insert(hf1);
      header_set.insert(hf2);

      // Instantiate an encoder context and a decoder context
      let mut http2_encoder: Encoder = Encoder::new(); 
      let mut http2_decoder: Decoder = Decoder::new(); 

      // Encode the header set
      let encoded_header_set: ~[u8] = http2_encoder.encode(header_set);

      // Decode the just encoded header set
      let decoded_header_set: ~HashSet<HeaderField> = http2_decoder.decode(encoded_header_set).unwrap();
  }
```

## Rust versions
This library is developed using Rust 0.10.

## Missing features
- [ ] Huffman encoding and decoding
- [ ] Change *header table size* of the encoding context

## License
This library is dual licensed under the MIT license and the Apache license (version 2.0).  
See LICENSE-APACHE and LICENSE-MIT for details.
