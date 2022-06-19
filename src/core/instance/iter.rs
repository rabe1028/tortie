use crate::core::invariant::Invariant;

use std::{marker::PhantomData, slice::Iter};


impl<'a, A> Invariant<'a> for Iter<'a, A>
where
    A: 'a,
    <Self as Iterator>::Item: 'a,
{
    type Domain = <Self as Iterator>::Item;
    type InvariantF<B>
    where
        Self::InvariantF<B>: Invariant<'a, Domain = B>,
        B: 'a,
    = IMap<Self, Box<dyn Fn(Self::Domain) -> B + 'a>, Box<dyn Fn(B) -> Self::Domain + 'a>, B>;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B + 'a,
        g: impl Fn(B) -> Self::Domain + 'a,
    ) -> Self::InvariantF<B> {
        let f = Box::new(f);
        let g = Box::new(g);

        IMap {
            iter: self,
            f,
            g,
            _phantom: PhantomData,
        }
    }
}

pub struct IMap<I, F, G, A>
where
    I: Iterator,
    F: Fn(I::Item) -> A,
    G: Fn(A) -> I::Item,
{
    pub(crate) iter: I,
    f: F,
    g: G,
    _phantom: PhantomData<A>,
}

impl<'a, I, F, G, A> Invariant<'a> for IMap<I, F, G, A>
where
    A: 'a,
    I: Iterator + Invariant<'a, Domain = I::Item>,
    F: Fn(I::Item) -> A + 'a,
    G: Fn(A) -> I::Item + 'a,
{
    type Domain = F::Output;
    type InvariantF<B>
    where
        B: 'a,
    = IMap<Self, Box<dyn Fn(Self::Domain) -> B + 'a>, Box<dyn Fn(B) -> Self::Domain + 'a>, B>;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B + 'a,
        g: impl Fn(B) -> Self::Domain + 'a,
    ) -> Self::InvariantF<B> {
        let f = Box::new(f);
        let g = Box::new(g);
        IMap {
            iter: self,
            f,
            g,
            _phantom: PhantomData,
        }
    }
}

impl<I, F, G, A> Iterator for IMap<I, F, G, A>
where
    I: Iterator,
    F: Fn(I::Item) -> A,
    G: Fn(A) -> I::Item,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(v) => Some((self.f)(v)),
        }
    }
}
