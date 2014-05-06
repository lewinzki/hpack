/* 
 * This module handles integer representations according to the HPACK specifications (Draft 07).
 * http://tools.ietf.org/html/draft-ietf-httpbis-header-compression-07
 */

/*
 * if I < 2^N - 1, encode_int I on N bits
 * else
 *     encode_int (2^N - 1) on N bits
 *     I = I - (2^N - 1)
 *     while I >= 128
 *         encode_int (I % 128 + 128) on 8 bits
 *         I = I / 128
 *     encode_int I on 8 bits
 */
pub fn encode_int(i: uint, n: u8) -> ~[u8] {
    let mut buffer: ~[u8] = ~[];
    let mut _i: uint = i;
    let bound: uint = (1 << n) - 1; // (2^N - 1)

    if i < bound {
        buffer.push(i.to_u8().unwrap());
    } else {
        buffer.push(bound.to_u8().unwrap());

        _i = _i - bound;

        while _i >= 128 {
            buffer.push(((_i % 128) + 128).to_u8().unwrap());
            _i = _i / 128;
        }

        buffer.push(_i.to_u8().unwrap());
    }

    buffer
}

/*
 * Takes ownership of a byte vector. Consumes bytes when decoding into an integer.
 * Returns a tuple of the integer and the byte vector (returning the ownership)
 *
 *
 * if I < 2^N - 1, return I
 * else
 *     M = 0
 *     repeat
 *         B = next octet
 *         I = I + (B & 127) * 2^M
 *         M = M + 7
 *     while B & 128 == 128
 *     return I
 */
pub fn decode_int(mut buffer: ~[u8], n: u8) -> Option<(uint, ~[u8])> {
    // The buffer must not be empty
    // N must be in the range [1; 8]
    if buffer.len() == 0 || n < 1 || n > 8 {
        return None;
    }

    let bound: uint = (1 << n) - 1; // (2^N - 1)

    buffer[0] = buffer[0] & bound.to_u8().unwrap(); // Mask N bits
    
    let mut i: uint = buffer.shift().unwrap().to_uint().unwrap();

    if i < bound {
        return Some((i, buffer));
    } else {
        let mut m: uint = 0;

        loop {
            // Match to see if the buffer is "prematurely" empty
            let buffer: uint = match buffer.shift() {
                Some(num) => num.to_uint().unwrap(),
                None => return None,
            };
            // TODO: From the draft: "Excessively large integer encodings - in value or octet length - MUST be treated as a decoding error."
            //       At the moment, Rust do not check for overflow. This might introduce some very weird errors. 
            i = i + (buffer & 127) * (1 << m);
            m = m + 7;

            if (buffer & 128) != 128 {
                break;
            }
        }

        Some((i, buffer))
    }
}

#[test]
fn encode_int_test() {
    let t0 = encode_int(10, 5);
    assert!(t0[0] == 10);

    let t1 = encode_int(1337, 5);
    assert!(t1[0] == 31);
    assert!(t1[1] == 154);
    assert!(t1[2] == 10);

    let t2 = encode_int(42, 8);
    assert!(t2[0] == 42);
}

#[test]
fn decode_int_test() {
    let b0 = ~[10];
    let (t0, _) = decode_int(b0, 5).unwrap();
    assert!(t0 == 10);

    let b1 = ~[31, 154, 10];
    let (t1, _) = decode_int(b1, 5).unwrap();
    assert!(t1 == 1337);

    let b2 = ~[42];
    let (t2, _) = decode_int(b2, 8).unwrap();
    assert!(t2 == 42);
}

#[test]
fn encode_int_decode_int_test() {
    let t0 = 123456789;
    let p0 = 6;
    let (_t0, _) = decode_int(encode_int(t0, p0), p0).unwrap();
    assert!(_t0 == t0);

    let t1 = 100;
    let p1 = 1;
    let (_t1, _) = decode_int(encode_int(t1, p1), p1).unwrap();
    assert!(_t1 == t1);

    let t2 = 22222222;
    let p2 = 3;
    let (_t2, _) = decode_int(encode_int(t2, p2), p2).unwrap();
    assert!(_t2 == t2);

    let t3 = 42;
    let p3 = 8;
    let (_t3, _) = decode_int(encode_int(t3, p3), p3).unwrap();
    assert!(_t3 == t3);


    // encode_int two numbers into the same buffer
    // and test that we can extract them one by one
    let t4 = 3999;
    let t5 = 4000;
    let b0 = encode_int(t4, 6);
    let b1 = encode_int(t5, 7);

    let mut b2 = ~[];
    b2.push_all_move(b0);
    b2.push_all_move(b1);

    let (t6, b2) = decode_int(b2, 6).unwrap();
    let (t7, _) = decode_int(b2, 7).unwrap();

    assert!(t6 == t4);
    assert!(t7 == t5);
}