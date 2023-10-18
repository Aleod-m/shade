#[macro_use]
pub mod combinator;
pub mod state;

#[derive(Debug)]
pub enum ParserRes {
    Succ,
    Fail,
}

/// The type of a parser.
pub trait Parser: Fn(&mut state::ParserState) -> ParserRes {}

impl<T> Parser for T
where
    T: Fn(&mut state::ParserState) -> ParserRes,
{ }

/// A dsl for writing most of the boilerplate while writing your grammar.
#[macro_export]
macro_rules! mk_parsers {
    (
    input = $input:ident;
    $($(#[doc= $doc:expr])*$v:vis $name:ident($($param:ident : $pty:ty),*) = $rule:tt);+
    ) => {
        $(
            $(#[doc = $doc])*
            $v fn $name($($param:$pty),*) -> impl crate::parser::internals::Parser {
                #[allow(unused_mut)]
                move |mut $input: &mut crate::parser::internals::state::ParserState| {
                    let state = $input.save();
                    mk_rule!($input, $rule, state)
                }
                
            }
        )+
    };
}

macro_rules! mk_rule {
    (
        $in:ident,
        ( seq:
            $($p:expr => { $($op:ident $($val:expr)?);* })*
            $(; after { $($op_:ident $($val_:expr)?);* })?
        )
        , $state:ident
    ) => { {
        $(
            match $p($in) {
                crate::parser::ParserRes::Succ => { $(mk_seq!($in, $op $($val)?));* },
                f @ crate::parser::ParserRes::Fail => {
                    $in.restore($state);
                    return f;
                }
            }
        )*
        $(
            $(mk_seq!($in, $op_ $($val_)?));*;
        )?
            return crate::parser::ParserRes::Succ; 
    } };
    (
        $in:ident,
        (choice:
            $(; $p:expr)*
        )
        , $state:ident
    ) => { {
        $(
            if let crate::parser::ParserRes::Succ = $p($in) {
                return crate::parser::ParserRes::Succ; 
            }
        )*
            $in.restore($state);
            return crate::parser::ParserRes::Fail; 
        } };
    (
        $in:ident,
        (expr :
            $p:expr
        )
        , $state:ident
    ) => { {
        let res = $p;
        if let crate::parser::ParserRes::Fail = res {
            $in.restore($state);
        }
        res
    } }
}

macro_rules! mk_seq {
    ($in: ident, push $val:expr) => {
        $in.push_node($val)
    };
    ($in: ident, stack $val:expr) => {
        $in.stack_node($val)
    };
    ($in: ident, pop) => {
        $in.pop_node()
    };
}
