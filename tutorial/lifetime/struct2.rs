// https://zhuanlan.zhihu.com/p/104742696

struct Parser {
    // ...
}

pub struct ParseIter<'a> {
    parser: &'a mut Parser,
    buf: &'a [u8],
}

impl Parser {
    pub fn parse<'a>(&'a mut self, buf: &'a [u8]) -> Option<&'a u8> {
        todo!()
    }

    pub fn iter<'a>(&'a mut self, buf: &'a [u8]) -> ParseIter<'a> {
        ParseIter { parser: self, buf }
    }
}

impl<'a> Iterator for ParseIter<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        return self.parser.parse(self.buf);
    }
}

fn main() {}
