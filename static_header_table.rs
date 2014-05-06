// HPACK Static Table Draft 07:
// http://tools.ietf.org/html/draft-ietf-httpbis-header-compression-07#appendix-B

use header_field::HeaderField;

pub struct StaticHeaderTable {
    fields: ~[HeaderField]
}

impl StaticHeaderTable {
    pub fn new() -> StaticHeaderTable {
        StaticHeaderTable {
            fields: ~[
            /* 1*/ HeaderField::new(~":authority", ~""),
            /* 2*/ HeaderField::new(~":method", ~"GET"),
            /* 3*/ HeaderField::new(~":method", ~"POST"),
            /* 4*/ HeaderField::new(~":path", ~"/"),
            /* 5*/ HeaderField::new(~":path", ~"/index.html"),
            /* 6*/ HeaderField::new(~":scheme", ~"http"),
            /* 7*/ HeaderField::new(~":scheme", ~"https"),
            /* 8*/ HeaderField::new(~":status", ~"200"),
            /* 9*/ HeaderField::new(~":status", ~"204"),
            /*10*/ HeaderField::new(~":status", ~"206"),
            /*11*/ HeaderField::new(~":status", ~"304"),
            /*12*/ HeaderField::new(~":status", ~"400"),
            /*13*/ HeaderField::new(~":status", ~"404"),
            /*14*/ HeaderField::new(~":status", ~"500"),
            /*15*/ HeaderField::new(~"accept-charset", ~""),
            /*16*/ HeaderField::new(~"accept-encoding", ~""),
            /*17*/ HeaderField::new(~"accept-language", ~""),
            /*18*/ HeaderField::new(~"accept-ranges", ~""),
            /*19*/ HeaderField::new(~"accept", ~""),
            /*20*/ HeaderField::new(~"access-control-allow-origin", ~""),
            /*21*/ HeaderField::new(~"age", ~""),
            /*22*/ HeaderField::new(~"allow", ~""),
            /*23*/ HeaderField::new(~"authorization", ~""),
            /*24*/ HeaderField::new(~"cache-control", ~""),
            /*25*/ HeaderField::new(~"content-disposition", ~""),
            /*26*/ HeaderField::new(~"content-encoding", ~""),
            /*27*/ HeaderField::new(~"content-language", ~""),
            /*28*/ HeaderField::new(~"content-length", ~""),
            /*29*/ HeaderField::new(~"content-location", ~""),
            /*30*/ HeaderField::new(~"content-range", ~""),
            /*31*/ HeaderField::new(~"content-type", ~""),
            /*32*/ HeaderField::new(~"cookie", ~""),
            /*33*/ HeaderField::new(~"date", ~""),
            /*34*/ HeaderField::new(~"etag", ~""),
            /*35*/ HeaderField::new(~"expect", ~""),
            /*36*/ HeaderField::new(~"expires", ~""),
            /*37*/ HeaderField::new(~"from", ~""),
            /*38*/ HeaderField::new(~"host", ~""),
            /*39*/ HeaderField::new(~"if-match", ~""),
            /*40*/ HeaderField::new(~"if-modified-since", ~""),
            /*41*/ HeaderField::new(~"if-none-match", ~""),
            /*42*/ HeaderField::new(~"if-range", ~""),
            /*43*/ HeaderField::new(~"if-unmodified-since", ~""),
            /*44*/ HeaderField::new(~"last-modified", ~""),
            /*45*/ HeaderField::new(~"link", ~""),
            /*46*/ HeaderField::new(~"location", ~""),
            /*47*/ HeaderField::new(~"max-forwards", ~""),
            /*48*/ HeaderField::new(~"proxy-authenticate", ~""),
            /*49*/ HeaderField::new(~"proxy-authorization", ~""),
            /*50*/ HeaderField::new(~"range", ~""),
            /*51*/ HeaderField::new(~"referer", ~""),
            /*52*/ HeaderField::new(~"refresh", ~""),
            /*53*/ HeaderField::new(~"retry-after", ~""),
            /*54*/ HeaderField::new(~"server", ~""),
            /*55*/ HeaderField::new(~"set-cookie", ~""),
            /*56*/ HeaderField::new(~"strict-transport-security", ~""),
            /*57*/ HeaderField::new(~"transfer-encoding", ~""),
            /*58*/ HeaderField::new(~"user-agent", ~""),
            /*59*/ HeaderField::new(~"vary", ~""),
            /*60*/ HeaderField::new(~"via", ~""),
            /*61*/ HeaderField::new(~"www-authenticate", ~""),
            ]
        }
    }

    // Return the header field at 'index' - 1 from the static header table
    // Returns None if out of bounds
    // Remeber, HPACK uses 1-indexing!!!
    pub fn get(&self, index: uint) -> Option<HeaderField> {
        if index < 1 || index > self.fields.len() {
            return None;
        }

        Some(self.fields[index - 1].clone())
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
}