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
    $(
        $(#[doc= $doc:expr])*
        $v:vis $name:ident ( $($param:ident : $pty:ty),* ) = $rule:tt
    );+ 
    $(;)?
    ) => {
        $(
            $(#[doc = $doc])*
            $v fn $name($($param:$pty),*) -> impl crate::parser::internals::Parser {
                #[allow(unused_mut)]
                move |mut $input: &mut crate::parser::internals::state::ParserState| {
                    let state = $input.save();
                    mk_rule!($input, state, $rule)
                }
                
            }
        )+
    };
}

macro_rules! mk_rule {
    // Simple rule.
    (
        $in:ident,
        $state:ident,
        { just $j_content:tt $(then $t_content:tt)? }
    ) => { {
        mk_just!($in, $state, $j_content)
    } };

    // Sequence rule.
    (
        $in:ident,
        $state:ident,
        { 
            seq $s_content:tt
            $(then $ops:tt)?
        }
    ) => { {
        mk_seq!($in, $state, $s_content);
        $( mk_ops!($in, $ops); )?
        return crate::parser::ParserRes::Succ;
    } };
    // Choice rule.
    (
        $in:ident,
        $state:ident,
        {
            choice $c_content:tt
            $( then $ops:tt )?
        }
    ) => { {
        mk_choice!($in, $c_content);
        $( mk_ops!($in, $ops); )?
        $in.restore($state);
        return crate::parser::ParserRes::Fail;
    } };
    // Expresion rule.
    (
        $in:ident,
        $state:ident,
        {
            $expr:expr
        }
    ) => { { 
        let res = $expr;
        match res {
            f @ crate::parser::ParserRes::Succ => f,
            f @ crate::parser::ParserRes::Fail => {
                $in.restore($state);
                f
            }
       }
    } };
}

macro_rules! mk_just {
    ($in:ident, $state:ident, { $p:expr => $($op:tt)? }) => { {
            let crate::parser::ParserRes::Succ = $p($in) else {
                $in.restore($state);
                return crate::parser::ParserRes::Fail;
            };
            $(mk_ops!($in, $op);)?
            return crate::parser::ParserRes::Succ;
    } }
}
macro_rules! mk_seq {
    ($in:ident, $state:ident, { $($p:expr $(=> $op:tt)?),* $(,)? }) => { {
        $(
            let crate::parser::ParserRes::Succ = $p($in) else {
                    $in.restore($state);
                    return crate::parser::ParserRes::Fail;
            };
            $(mk_ops!($in, $op);)?
        )*
    } };
}
macro_rules! mk_choice {
    ($in:ident, { $($p:expr $(=> $op:tt)?),* $(,)? }) => { {
        $(
            if let res @ crate::parser::ParserRes::Succ = $p($in) {
                return res;
            }
            $(mk_ops!($in, $op);)?
        )*
    } };
}

macro_rules! mk_ops {
    ($in: ident, ()) => { };
    ($in: ident, { $($op:ident $($val:expr)?);*}) => { 
        $(mk_op!($in, $op $($val)?);)*
    };
}

macro_rules! mk_op {
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
