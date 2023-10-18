use std::fmt::Debug;

use thiserror::Error;

use crate::{lexer::{LexedBuffer, self, TkHandle, token::TkKind}, ast::{self, NodeKind, NodeHandle}, utils::IVec};

use super::Parser;

#[derive(Error, Debug)]
pub enum StateError
{
    #[error("lexer error.")]
    LexError(#[from] lexer::LexError),
    #[error("the parser tried to reset its position farther than it was.")]
    OOBReset,
    #[error("The parser tried to check a null token can happen if the input is empty or the parser reaches the end of a file.")]
    TriedToCheckNullKind,
    #[error("Syntax Error: {_0}")]
    ParseError(#[from] super::super::ParseError),
}


#[derive(Debug)]
pub struct ParserState {
    input: LexedBuffer,
    tk_handle: Option<TkHandle>,
    kinds: Vec<ast::NodeKind>,
    tks: Vec<TkHandle>,
    stack: Vec<(ast::NodeKind, TkHandle)>,
    errors: Vec<StateError>,
}

pub struct Save(usize, usize, usize);

impl ParserState
{
    pub fn new(input: LexedBuffer) -> Self {
        let kinds = Vec::with_capacity(input.nb_tokens());
        let tks = Vec::with_capacity(input.nb_tokens());
        let first = input.first();
        Self {
            input,
            kinds,
            tks,
            tk_handle: first, 
            stack: vec![],
            errors: Vec::new(),
        }
    }

    pub fn save(&self) -> Save {
        Save(self.kinds.len(), self.stack.len(), self.errors.len())
    }

    pub fn restore(&mut self, Save(kl, sl, el): Save) {
        self.kinds.truncate(kl);
        self.stack.truncate(sl);
        self.errors.truncate(el);
    }
    
    pub fn is_kind(&self, kind: TkKind) -> bool {
        if let Some(h) = self.tk_handle {
            *self.input.get_kind(h) == kind
        } else {

            false
        }
    }

    pub fn push_node(&mut self, kind: NodeKind) {
        self.kinds.push(kind);
        // It is safe to
        self.tks.push(self.tk_handle.expect("Pushed a Node kind without cheking if the token was valid."));
        self.tk_handle = self.tk_handle.and_then(|h| self.input.next_handle(h));
    }

    pub fn stack_node(&mut self, kind: NodeKind) {
        self.stack.push((kind, self.tk_handle.expect("Pushed a Node kind without checking if the token was valid.")));
        self.tk_handle = self.tk_handle.and_then(|h| self.input.next_handle(h));
    }

    pub fn pop_node(&mut self) {
        let (k, tk) = self.stack.pop().expect("Poped the empty stack.");
        self.kinds.push(k);
        self.tks.push(tk);
    }

    pub fn run_parser(mut self, p: impl Parser) -> ParsedBuffer {
        let res = p(&mut self);
        ParsedBuffer {
            input: self.input,
            nodes: self.kinds.into(),
            handles: self.tks.into(),
            errors: self.errors.into(),
        }
    }

    pub fn run_parser_and_reset(&mut self, p: impl Parser) {

    }
}

pub struct ParsedBuffer {
    input: LexedBuffer,
    nodes: IVec<NodeKind>,
    handles: IVec<TkHandle>,
    errors: IVec<StateError>,
}
impl ParsedBuffer {
    pub fn nodes(&self) -> IVec<NodeKind> {
        self.nodes.clone()
    }
}

