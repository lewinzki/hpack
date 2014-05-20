#[cfg(test)]
mod test {
    use collections::hashmap::HashSet;
    use integer_representation::encode_int; 

    use header_field::HeaderField;
    use encoder::Encoder;
    use decoder::Decoder;

    #[test]
    fn test_hpack() { 
        let mut hpack_decoder = ~Decoder::new();
        let mut hpack_encoder = ~Encoder::new();

        /*
         * Test a named literal with index
         * Test an indexed literal in the static header table
         */
        let mut hb0 = ~HashSet::new();
        let h0 = HeaderField::new(~"Foo", ~"Bar");
        let h1 = HeaderField::new(~":authority", ~"Respect my authoritah!!!!");
        hb0.insert(h0.clone());
        hb0.insert(h1.clone());

        let hs0_encoded = hpack_encoder.encode(hb0);
        let hs0_decoded = hpack_decoder.decode(hs0_encoded.clone()).unwrap();

        assert!(hs0_decoded.get(&h0.key)[0] == h0.value);
        assert!(hs0_decoded.get(&h1.key)[0] == h1.value);


        /*
         * h2: Use a field already in the header table - should use an indexed header
         * h3: Use a field in the static header table - should use an indexed header
         */
        let mut hb1 = ~HashSet::new();
        let h2 = HeaderField::new(~"Foo", ~"Bar");
        let h3 = HeaderField::new(~":status", ~"200");
        let h4 = HeaderField::new(~"Baz", ~"Hello World!!!");
        hb1.insert(h2.clone());
        hb1.insert(h3.clone());
        hb1.insert(h4.clone());

        let hs1_encoded = hpack_encoder.encode(hb1);
        let hs1_decoded = hpack_decoder.decode(hs1_encoded.clone()).unwrap();

        assert!(hs1_decoded.get(&h2.key)[0] == h2.value);
        assert!(hs1_decoded.get(&h3.key)[0] == h3.value);
        assert!(hs1_decoded.get(&h4.key)[0] == h4.value);

        /*
         * Test a mix of header fields. Some in header table, some in static header table etc.
         */
        let mut hb2 = ~HashSet::new();
        let h5  = HeaderField::new(~":status", ~"200");
        let h6  = HeaderField::new(~"www-authenticate", ~"Basic");
        let h7  = HeaderField::new(~":server", ~"RustyHTTP");
        let h8  = HeaderField::new(~"date", ~"04-04-2014");
        let h9  = HeaderField::new(~":status", ~"200");
        let h10 = HeaderField::new(~"Foo", ~"Bar");
        let h11 = HeaderField::new(~"Baz", ~"Goodbye World!!!");
        let h12 = HeaderField::new(~"cookie", ~"y2tqg67f8g8437qfg9867t487&/(%/GFCih37824/)dsfhasdhfuisdhf/#RYhffasøæådasdnhaksh");
        let h13 = HeaderField::new(~"Doge", ~"much awesome");
        let h14 = HeaderField::new(~"_haps_snaps_,.", ~"Vi sejled' op ad å'en!!!");
        let h15 = HeaderField::new(~"Proin vel pellentesque nibh. Aliquam vehicula arcu sit amet nibh lobortis cursus. Fusce dignissim felis vel velit gravida, vitae pellentesque orci fringilla. Nunc nec tempor neque. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a est nisi. Duis lobortis sodales imperdiet. Maecenas vitae consequat justo. Donec sit amet laoreet ipsum. Nam euismod purus sed elementum condimentum. Sed vitae lorem eu est egestas mattis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Suspendisse quis elit dignissim, scelerisque libero vel, tempor odio.", ~"Ipsum Lorem");
        let h16 = HeaderField::new(~"Lorem ipsum", ~"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer at justo non sapien fringilla facilisis in at augue. Sed interdum, leo vel hendrerit fermentum, mi dolor malesuada tellus, in lobortis nisl sem porta augue. Praesent pellentesque, quam eget suscipit consequat, arcu velit mattis est, et porttitor nulla arcu eget velit. Nam at sodales massa. Vivamus velit erat, accumsan nec velit in, mattis euismod massa. Sed vel enim ullamcorper, lobortis mauris a, tempor metus. Cras ullamcorper odio ac odio bibendum rutrum. Duis blandit faucibus risus, sit amet fringilla odio venenatis et. Phasellus vehicula odio varius eros ultrices elementum. In rhoncus enim a massa interdum convallis.");
        
        hb2.insert(h5.clone());
        hb2.insert(h6.clone());
        hb2.insert(h7.clone());
        hb2.insert(h8.clone());
        hb2.insert(h9.clone());
        hb2.insert(h10.clone());
        hb2.insert(h11.clone());
        hb2.insert(h12.clone());
        hb2.insert(h13.clone());
        hb2.insert(h14.clone());
        hb2.insert(h15.clone());
        hb2.insert(h16.clone());

        let hs2_encoded = hpack_encoder.encode(hb2);
        let hs2_decoded = hpack_decoder.decode(hs2_encoded.clone()).unwrap();

        assert!(hs2_decoded.get(&h5.key)[0] == h5.value);
        assert!(hs2_decoded.get(&h6.key)[0] == h6.value);
        assert!(hs2_decoded.get(&h7.key)[0] == h7.value);
        assert!(hs2_decoded.get(&h8.key)[0] == h8.value);
        assert!(hs2_decoded.get(&h9.key)[0] == h9.value);
        assert!(hs2_decoded.get(&h10.key)[0] == h10.value);
        assert!(hs2_decoded.get(&h11.key).len() == 2);
        assert!(hs2_decoded.get(&h12.key)[0] == h12.value);
        assert!(hs2_decoded.get(&h13.key)[0] == h13.value);
        assert!(hs2_decoded.get(&h14.key)[0] == h14.value);
        assert!(hs2_decoded.get(&h15.key)[0] == h15.value);
        assert!(hs2_decoded.get(&h16.key)[0] == h16.value);
    }

    #[test]
    fn test_bug() {
        let mut hpack_decoder = ~Decoder::new();

        let h = HeaderField::new(~":authority", ~"Respect my authoritah!!!!");
        let mut index = encode_int(1, 6);
        let value = ~"Respect my authoritah!!!!";
        let value_length = encode_int(value.clone().len(), 7);
        let mut frame: ~[u8] = ~[];

        index[0] |= 64;
        frame.push_all_move(index);
        frame.push_all_move(value_length);
        frame.push_all_move(value.clone().into_bytes());

        let header_fields = hpack_decoder.decode(frame.clone()).unwrap();
        let header_fields2 = hpack_decoder.decode(~[]).unwrap();

        assert!(header_fields.get(&h.key)[0] == value.clone());
        assert!(header_fields2.get(&h.key)[0] == value);
    }
}