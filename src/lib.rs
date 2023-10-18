pub mod ast;
pub mod parser;
pub mod lexer;
//pub mod types;

mod utils {
    use std::fmt;
    use std::ops::Range;

    pub type IStr = std::rc::Rc<str>;
    pub type IVec<T> = std::rc::Rc<[T]>;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Loc {
        pub(crate) pos: usize,
        pub row: usize,
        pub col: usize,
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Span {
        pub start: Loc,
        pub end: Loc,
    }

    impl Span {
        pub fn new(start: Loc, end: Loc) -> Self {
            Self { start, end }
        }

        pub(crate) fn to_range(&self) -> Range<usize> {
            self.start.pos..self.end.pos
        }
    }

    impl fmt::Display for Span {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "start: {}\tend: {}", self.start, self.end)
        }
    }

    impl fmt::Display for Loc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.row, self.col)
        }
    }
}
