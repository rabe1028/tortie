use crate::core::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal,
};

impl<'a, A> Invariant<'a> for Option<A> {
    type Domain = A;
    type InvariantF<B>
    where
        B: 'a,
    = Option<B>;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::InvariantF<B> {
        self.map(f)
    }
}

impl<A> Functor<'_> for Option<A> {
    type FunctorF<B> = Option<B>;

    fn map<B>(self, f: impl FnOnce(Self::Domain) -> B) -> Self::FunctorF<B> {
        self.map(f)
    }
}

impl<A> Semigroupal for Option<A> {
    type From = A;
    type SemigroupalF<B> = Option<B>;

    fn product<B>(self, other: Self::SemigroupalF<B>) -> Self::SemigroupalF<(Self::From, B)> {
        self.zip(other)
    }
}

impl<'a, F, A, B> Apply<'a, A, B> for Option<F>
where
    F: FnOnce(A) -> B,
{
    type ApplyF<D> = Option<D>;
    fn ap(self, fa: Self::ApplyF<A>) -> Self::ApplyF<B> {
        self.map(|f: F| fa.map(f)).flatten()
    }
}

impl<'a, A> InvariantMonoidal<'a> for Option<A> {
    fn unit() -> Self::InvariantF<()> {
        Some(())
    }
}

impl<'a, A> Applicative<'a> for Option<A> {
    fn pure(x: Self::Domain) -> Self {
        Some(x)
    }
}

impl<'a, A> FlatMap<'a> for Option<A> {
    fn flat_map<B>(self, f: impl FnOnce(Self::Domain) -> Self::FunctorF<B>) -> Self::FunctorF<B> {
        match self {
            Some(x) => f(x),
            None => None,
        }
    }

    fn tailrec<U>(a: U, f: impl Fn(U) -> Self::FunctorF<Result<Self::Domain, U>>) -> Self {
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
