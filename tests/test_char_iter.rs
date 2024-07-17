use std::str::Chars;

use shear::iter::Limited;

pub struct TestIter<'a> {
    chars: Chars<'a>,
}

impl<'a> From<Chars<'a>> for TestIter<'a> {
    fn from(chars: Chars<'a>) -> Self {
        Self { chars }
    }
}

impl<'a> Limited for TestIter<'a> {
    type ContdIter = std::array::IntoIter<char, 3>;

    fn contd() -> Self::ContdIter {
        ['.', '.', '.'].into_iter()
    }

    fn element_size(_: &Self::Item) -> usize {
        1
    }
}

impl<'a> Iterator for TestIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}
