use crate::core::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal,
};

impl<'a, A: 'a> Invariant<'a> for Option<A> {
    type Domain = A;
    type Rebind<B>
        = Option<B>
    where
        B: 'a;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::Rebind<B> {
        self.map(|a| f(a))
    }
}

impl<'a, A: 'a> Functor<'a> for Option<A> {
    fn map<B: 'a>(self, f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B> {
        self.map(f)
    }
}

impl<'a, A: 'a> Semigroupal<'a> for Option<A> {
    fn product<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Domain: Clone,
        B: Clone,
    {
        self.zip(other)
    }
}

impl<'a, F> Apply<'a> for Option<F>
where
    F: 'a,
{
    fn ap<A: 'a + Clone, B: 'a>(self, fa: Self::Rebind<A>) -> Self::Rebind<B>
    where
        Self::Domain: FnMut(A) -> B,
        Self::Rebind<A>: Functor<'a>,
        Self::Rebind<B>: Functor<'a>,
    {
        self.map(|f: F| fa.map(f)).flatten()
    }
}

impl<'a, A: 'a> InvariantMonoidal<'a> for Option<A> {
    fn unit() -> Self::Rebind<()> {
        Some(())
    }
}

impl<'a, A: 'a> Applicative<'a> for Option<A> {
    fn pure(x: Self::Domain) -> Self {
        Some(x)
    }
}

impl<'a, A: 'a> FlatMap<'a> for Option<A> {
    fn flat_map<B: 'a>(
        self,
        mut f: impl FnMut(Self::Domain) -> Self::Rebind<B>,
    ) -> Self::Rebind<B> {
        match self {
            Some(x) => f(x),
            None => None,
        }
    }

    fn tailrec<U: 'a>(a: U, mut f: impl FnMut(U) -> Self::Rebind<Result<Self::Domain, U>>) -> Self {
        // this code happend stack overflow.
        // match f(a) {
        //     None => None,
        //     Some(Err(a1)) => Option::tailrec(a1, f),
        //     Some(Ok(b)) => Some(b),
        // }
        let mut a = a;
        for _ in 0..Self::TAILREC_LIMIT {
            match f(a) {
                None => return None,
                Some(Err(a1)) => a = a1,
                Some(Ok(b)) => return Some(b),
            }
        }
        unreachable!("Tailrec limit reached!!!");
    }
}
