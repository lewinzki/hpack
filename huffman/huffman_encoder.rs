use std::num;

use huffman::huffman_codes::HUFFMAN_CODES;

pub struct HuffmanEncoder {
    result: ~[u8],
}

impl HuffmanEncoder {
    pub fn new() -> HuffmanEncoder {
        HuffmanEncoder {
            result: ~[],
        }
    }

    pub fn encode(&mut self, bytes: ~[u8]) -> ~[u8] {
        self.result = ~[];

        let mut space = 8;

        if bytes.len() == 0 {
            return bytes;
        }

        for byte in bytes.iter() {
            let mut code: uint = num::from_str_radix(HUFFMAN_CODES[*byte].clone(), 2).unwrap();
            let mut length = HUFFMAN_CODES[*byte].len();

            while length != 0 {
                if space >= length {
                    self.add_code(code << (space - length), space);
                    code = 0;
                    space -= length;
                    length = 0;
                } else {
                    let shift = length - space;
                    let msb = code >> shift;
                    self.add_code(msb, space);
                    code -= msb << shift;
                    length -= space;
                    space = 0;
                }

                if space == 0 {
                    space = 8;
                }
            }
        }

        if space != 8 {
            let code: uint = num::from_str_radix(HUFFMAN_CODES[256].clone(), 2).unwrap();
            self.add_code(code >> (HUFFMAN_CODES[256].len() - space), space);
        }

        self.result.clone()
    }


    fn add_code(&mut self, code: uint, space: uint) {
        if space == 8 {
            self.result.push(code.to_u8().unwrap());
        } else {
            self.result[self.result.len() - 1] |= code.to_u8().unwrap();
        }
    }    
}


#[test]
fn huffman_encoder_test() {
    let mut encoder = HuffmanEncoder::new();
    let mut ascii_bytes: ~[u8] = ~[];

    for i in range(0, 256) {
        ascii_bytes.push(i.to_u8().unwrap());
    }

    let huffman_codes = ~[255, 255, 238, 191, 255, 251, 191, 255, 254, 243, 255, 255, 189, 255, 255, 239, 191, 255, 251, 255, 255, 255, 3, 255, 255, 193, 255, 255, 240, 191, 255, 252, 63, 255, 255, 19, 255, 255, 197, 255, 255, 241, 191, 255, 252, 127, 255,255, 35, 255, 255, 201, 255, 255, 242, 191, 255, 252, 191, 255, 255, 51, 255, 255, 205, 255, 255, 243, 191, 255, 252, 255, 255, 255, 67, 255, 255, 209, 255, 255, 244, 191, 255, 253, 63, 255, 255, 83, 255, 255, 213, 255, 255, 245, 191, 255,253, 127, 255, 255, 99, 255, 255, 217, 55, 255, 62, 31, 254, 127, 252, 123, 39,255, 127, 175, 143, 247, 254, 101, 204, 249, 192, 73, 16, 67, 20, 114, 75, 55, 103, 255, 242, 127, 255, 191, 239, 255, 236, 253, 189, 218, 59, 244, 234, 249, 120, 124, 254, 159, 93, 123, 60, 124, 190, 223, 125, 180, 121, 254, 63, 62, 159, 175, 223, 252, 255, 255, 246, 191, 239, 255, 187, 191, 255, 228, 239, 85, 43, 225, 85, 108, 245, 246, 178, 219, 155, 127, 243, 12, 93, 199, 151, 62, 157, 125, 255, 255, 191, 249, 255, 254, 255, 223, 255, 255, 111, 255, 255, 220, 255, 255, 247, 127, 255, 253, 239, 255, 255, 127, 255, 255, 224, 255, 255, 248, 127, 255, 254, 47, 255, 255, 143, 255, 255, 228, 255, 255, 249, 127, 255, 254, 111, 255, 255, 159, 255, 255, 232, 255, 255, 250, 127, 255, 254, 175, 255, 255, 175, 255, 255, 236, 255, 255, 251, 127, 255, 254, 239, 255, 255, 191, 255, 255, 240, 255, 255, 252, 127, 255, 255, 47, 255, 255, 207, 255, 255, 244, 255, 255, 253, 127, 255, 255, 111, 255, 255, 223, 255, 255, 248, 255, 255, 254, 127, 255, 255, 175, 255, 255, 239, 255, 255, 252, 255, 255, 255, 127, 255, 255, 239, 255, 255, 255, 255, 255, 1, 255, 255, 129, 255, 255, 193, 127, 255, 224, 255, 255, 240, 159, 255, 248, 95, 255, 252, 55, 255, 254, 31, 255, 255, 17, 255, 255, 137, 255, 255, 197,127, 255, 226, 255, 255, 241, 159, 255, 248, 223, 255, 252, 119, 255, 254, 63, 255, 255, 33, 255, 255, 145, 255, 255, 201, 127, 255, 228, 255, 255, 242, 159, 255, 249, 95, 255, 252, 183, 255, 254, 95, 255, 255, 49, 255, 255, 153, 255, 255,205, 127, 255, 230, 255, 255, 243, 159, 255, 249, 223, 255, 252, 247, 255, 254,127, 255, 255, 65, 255, 255, 161, 255, 255, 209, 127, 255, 232, 255, 255, 244, 159, 255, 250, 95, 255, 253, 55, 255, 254, 159, 255, 255, 81, 255, 255, 169, 255, 255, 213, 127, 255, 234, 255, 255, 245, 159, 255, 250, 223, 255, 253, 119, 255, 254, 191, 255, 255, 97, 255, 255, 177, 255, 255, 217, 127, 255, 236, 255, 255,246, 159, 255, 251, 95, 255, 253, 183, 255, 254, 223, 255, 255, 113, 255, 255, 185, 255, 255, 221, 127, 255, 238, 255, 255, 247, 159, 255, 251, 223, 255, 253, 247, 255, 254, 255, 255, 255, 129, 255, 255, 193, 255, 255, 225, 127, 255, 240, 255, 255, 248, 159, 255, 252, 95, 255, 254, 55, 255, 255, 31, 255, 255, 145, 255, 255, 201, 255, 255, 229, 127, 255, 242, 255, 255, 249, 159, 255, 252, 223, 255, 254, 119, 255, 255, 63, 255, 255, 161, 255, 255, 209, 255, 255, 233, 127, 255,244, 255, 255, 250, 159, 255, 253, 95, 255, 254, 183, 255, 255, 95, 255, 255, 177, 255, 255, 217, 255, 255, 237, 127, 255, 246, 255];
    let encoded_bytes: ~[u8] = encoder.encode(ascii_bytes);

    assert!(encoded_bytes == huffman_codes);
}