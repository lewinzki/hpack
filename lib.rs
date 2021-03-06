#![crate_id = "hpack#0.1-draft07"]

#![license = "MIT/ASL2"]

#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(macro_rules)]

extern crate collections;

// Reexport items for beautiful API
// (e.g. hpack::Decoder instead of hpack::decoder::Decoder)
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::header_field::HeaderField;
pub use self::header_collection::HeaderCollection;

mod encoder;
mod decoder;
mod header_field;
mod header_collection;
mod header_table;
mod header_set;
mod reference_set;
mod integer_representation;
mod static_header_table;
mod representation;
mod representation_encoder;
mod huffman {
    mod huffman_tree;
    mod huffman_codes;
    pub mod huffman_encoder;
    pub mod huffman_decoder;
}

mod test;