use crate::core::invariant::Invariant;

use std::{marker::PhantomData, slice::Iter};

impl<'a, A> Invariant<'a> for Iter<'a, A> {
    type Domain = <Self as Iterator>::Item;
    type InvariantF<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a> =
        IterableMap<Self, F, B>;

    fn imap<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>(
        self,
        f: F,
        _g: G,
    ) -> Self::InvariantF<B, F, G> {
        IterableMap {
            i: self,
            f: f,
            _phantom: PhantomData,
        }
    }
}

pub struct IterableMap<I, F, B>
where
    I: Iterator,
    F: Fn(I::Item) -> B,
{
    pub i: I,
    pub f: F,
    _phantom: PhantomData<B>,
}

impl<I, F, B> Iterator for IterableMap<I, F, B>
where
    I: Iterator,
    F: Fn(I::Item) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        match self.i.next() {
            None => None,
            Some(v) => Some((self.f)(v)),
        }
    }
}

impl<'a, I, F1, A> Invariant<'a> for IterableMap<I, F1, A>
where
    I: Iterator,
    F1: Fn(I::Item) -> A,
{
    type Domain = A;
    type InvariantF<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a> =
        IterableMap<Self, F, B>;

    fn imap<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>(
        self,
        f: F,
        g: G,
    ) -> Self::InvariantF<B, F, G> {
        IterableMap {
            i: self,
            f: f,
            _phantom: PhantomData,
        }
    }
}

// impl<'a, I, F1> Invariant<'a> for Map<I, F1>
// where
//     Self: Sized + Iterator,
//     F1: FnMut(<Self as Iterator>::Item),
// {
//     type Domain = F1::Output;
//     type InvariantF<B:'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>
//         = Map<Self, F>;

//     fn imap<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>(
//             self,
//             f: F,
//             g: G,
//         ) -> Self::InvariantF<B, F, G> {
//         self.map(f)
//     }
// }
