use crate::core::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal,
};

impl<'a, A: 'a> Invariant<'a> for Box<A> {
    type Domain = A;
    type Rebind<B>
        = Box<B>
    where
        B: 'a;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::Rebind<B> {
        Box::new(f(*self))
    }
}

impl<'a, A: 'a> Functor<'a> for Box<A> {
    fn map<B: 'a>(self, mut f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B> {
        Box::new(f(*self))
    }
}

impl<'a, A: 'a> Semigroupal<'a> for Box<A> {
    fn product<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Domain: Clone,
        B: Clone,
    {
        Box::new((*self, *other))
    }
}

impl<'a, F: 'a> Apply<'a> for Box<F> {
    fn ap<A: 'a + Clone, B: 'a>(self, fa: Self::Rebind<A>) -> Self::Rebind<B>
    where
        Self::Domain: FnMut(A) -> B,
        Self::Rebind<A>: Functor<'a>,
        Self::Rebind<B>: Functor<'a>,
    {
        let mut f = *self;
        Box::new(f(*fa))
    }
}

impl<'a, A: 'a> InvariantMonoidal<'a> for Box<A> {
    fn unit() -> Self::Rebind<()> {
        Box::new(())
    }
}

impl<'a, A: 'a> Applicative<'a> for Box<A> {
    fn pure(x: Self::Domain) -> Self {
        Box::new(x)
    }
}

impl<'a, A: 'a> FlatMap<'a> for Box<A> {
    fn flat_map<B: 'a>(
        self,
        mut f: impl FnMut(Self::Domain) -> Self::Rebind<B>,
    ) -> Self::Rebind<B> {
        f(*self)
    }

    fn tailrec<U: 'a>(a: U, mut f: impl FnMut(U) -> Self::Rebind<Result<Self::Domain, U>>) -> Self {
        let mut next = a;

        loop {
            match *f(next) {
                Ok(value) => return Box::new(value),
                Err(value) => next = value,
            }
        }
    }
}
