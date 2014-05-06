pub trait Representation {
    fn encode(&self) -> ~[u8];
}


/*   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 1 |        Index (7+)         |
 * +---+---------------------------+
 *        Indexed Header Field
 */
pub struct IndexedHeader {
    index: uint,
}

impl IndexedHeader {
    pub fn new(index: uint) -> IndexedHeader {
        IndexedHeader {
            index: index
        }
    }
}

/* 
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 1 |      Index (6+)       |
 * +---+---+---+-------------------+
 * | H |     Value Length (7+)     |
 * +-------------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field with Incremental Indexing - Indexed Name
 *
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 0 | 0 |  Index (4+)   |
 * +---+---+-----------------------+
 * | H |     Value Length (7+)     |
 * +---+---------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field without Indexing - Indexed Name
 *
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 0 | 1 |  Index (4+)   |
 * +---+---+-----------------------+
 * | H |     Value Length (7+)     |
 * +---+---------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field never Indexed - Indexed Name
 */
pub struct IndexedLiteral {
    indexing: bool,
    never_indexed: bool, 
    index: uint,
    value_huffman: bool,
    value_length: uint,
    value_string: ~[u8]
}

impl IndexedLiteral {
    pub fn new(indexing: bool, never_indexed: bool, index: uint, value_huffman: bool, value_string: ~[u8]) -> IndexedLiteral {
        IndexedLiteral {
            indexing: indexing,
            never_indexed: never_indexed, 
            index: index,
            value_huffman: value_huffman,
            value_length: value_string.len(),
            value_string: value_string
        }
    }
}

/* 
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 1 |           0           |
 * +---+---+---+-------------------+
 * | H |     Name Length (7+)      |
 * +-------------------------------+
 * |  Name String (Length octets)  |
 * +-------------------------------+
 * | H |     Value Length (7+)     |
 * +-------------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field with Incremental Indexing - New Name
 *
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 0 | 0 |       0       |
 * +---+---+-----------------------+
 * | H |     Name Length (7+)      |
 * +---+---------------------------+
 * |  Name String (Length octets)  |
 * +---+---------------------------+
 * | H |     Value Length (7+)     |
 * +---+---------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field without Indexing - New Name
 *
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 0 | 1 |       0       |
 * +---+---+-----------------------+
 * | H |     Name Length (7+)      |
 * +---+---------------------------+
 * |  Name String (Length octets)  |
 * +---+---------------------------+
 * | H |     Value Length (7+)     |
 * +---+---------------------------+
 * | Value String (Length octets)  |
 * +-------------------------------+
 * Literal Header Field never Indexed - New Name
 */
pub struct NamedLiteral {
    indexing: bool,
    never_indexed: bool,
    name_huffman: bool,
    name_length: uint,
    name_string: ~[u8],
    value_huffman: bool,
    value_length: uint,
    value_string: ~[u8]
}

impl NamedLiteral {
    pub fn new(indexing: bool, never_indexed: bool, name_huffman: bool, name_string: ~[u8], value_huffman: bool, value_string: ~[u8]) -> NamedLiteral {
        NamedLiteral {
            indexing: indexing,
            never_indexed: never_indexed,
            name_huffman: name_huffman,
            name_length: name_string.len(),
            name_string: name_string,
            value_huffman: value_huffman,
            value_length: value_string.len(),
            value_string: value_string
        }
    }
}


/*
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 1 | 1 |       0       |
 * +---+---------------------------+
 *       Reference Set Emptying 
 *
 *
 *   0   1   2   3   4   5   6   7
 * +---+---+---+---+---+---+---+---+
 * | 0 | 0 | 1 | 0 | Max size (4+) |
 * +---+---------------------------+
 * Maximum Header Table Size Change
 */
pub struct ContextUpdate {
    flag: bool, // true: empty reference set, false: change header table size
    data: uint
}

impl ContextUpdate {
    pub fn new(flag: bool, data: uint) -> ContextUpdate {
        ContextUpdate {
            flag: flag,
            data: data
        }
    }
}