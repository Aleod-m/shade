use super::{state, Parser, ParserRes};



#[macro_export]
macro_rules! ordered_choice {
    ( $p:ident ( $($pargs:expr),* ) ) => {
        $p( $($pargs),* )
    };
    (
        $p:ident ( $($pargs:expr),* ),
        $($ps:ident ( $($psargs:expr),* ) ),* $(,)?
    ) => {
        $crate::parser::combinator::or(
            $p ($($pargs),*),
            ordered_choice!( $( $ps( $($psargs),* ) ),* ) 
        )
    };
}

#[macro_export]
macro_rules! _do {
    (
        $in:ident;
        $($p:expr => { $($op:ident $($val:expr)?);* })*
        $(; after { $($op_:ident $($val_:expr)?);* })?
    )=> {
       $(
            match $p($in) {
                crate::parser::ParserRes::Succ => { $(internal_do!($in, $op $($val)?));* },
                f @ crate::parser::ParserRes::Fail => return f,
            }
        )*
        $(
            $(internal_do!($in, $op_ $($val_)?));*
        )?
    }
}

macro_rules! internal_do {
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
