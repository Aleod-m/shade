use std::{iter::Peekable, fmt::Debug};

pub trait MetaData {
    type InputMetaData;
    fn get(metadata: &Self::InputMetaData) -> &Self;
}

pub trait InputMetaData {
    type Item;
    fn update(&mut self, item: &Self::Item); 
}

#[derive(Debug)]
pub struct Input<I, IM>
where
    I: Iterator,
    I::Item: std::fmt::Debug,
    IM: InputMetaData<Item = I::Item>,
{
    iter: Peekable<I>,
    metadata: IM,
}

impl<I, IM> Input<I, IM> 
where
    I: Iterator,
    I::Item: std::fmt::Debug,
    IM: InputMetaData<Item = I::Item>,
{
    pub fn new(iter: I, metadata: IM) -> Self {
        Self { iter: iter.peekable(), metadata }
    }

    pub fn get<M>(&self) -> &M
    where
        M: MetaData<InputMetaData = IM>,
    {
        M::get(&self.metadata)
    }

    pub fn next(&mut self) -> Option<I::Item> {
        let item = self.iter.next()?;
        self.metadata.update(&item);
        Some(item)
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }

    /// Advance to the next Self::Item if it fullfil the predicate.
    #[must_use]
    pub fn next_if<F>(&mut self, pred: F) -> Option<I::Item>
    where
        F: Fn(&I::Item) -> bool,
    {
        // TODO: Change this to inspect when it is stabilized on options.
        let item = self.iter.next_if(|c| pred(c))?;
        self.metadata.update(&item);
        Some(item)
    }


    /// Advance to the next Self::Item if it is fullfill the predicate without returning it.
    pub fn skip_if<F>(&mut self, pred: F) -> Option<()>
    where
        F: Fn(&I::Item) -> bool,
    {
        self.next_if(pred).map(|_| ())
    }


    /// Skip Self::Items while they fullfil the predicate.
    pub fn skip_while<F>(&mut self, pred: F)
    where
        F: Fn(&I::Item) -> bool,
    {
        while self.next_if(&pred).is_some() {}
    }

}
    

impl<I, IM> Input<I, IM> 
where
    I: Iterator, 
    IM: InputMetaData<Item = I::Item>,
    I::Item: Eq + PartialEq + Debug,
{
    /// Returns true if the next char is the one expected.
    pub fn peek_is(&mut self, c: I::Item) -> bool {
        self.iter.peek().map(|c_| *c_ == c).unwrap_or(false)
    }
    /// Advance to the next Self::Item if it is the one expected.
    pub fn next_is(&mut self, c: I::Item) -> bool {
        self.next_if(|c_| *c_ == c).is_some()
    }
    /// Skip Self::Itemd while they aren't the one expected.
    pub fn skip_to_next(&mut self, c: I::Item) {
        self.skip_while(|c_| *c_ != c)
    }
    /// Advance to the next Self::Item if it is the one expected without returning it.
    pub fn skip_next(&mut self, c: I::Item) -> bool {
        self.skip_if(|c_| *c_ == c).is_some()
    }
}
