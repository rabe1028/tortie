use crate::core::{
    applicative::Applicative, apply::Apply, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal, flat_map::FlatMap,
};

impl<A, E> Invariant<'_> for Result<A, E> {
    type Domain = A;
    type InvariantF<B> = Result<B, E>;

    fn imap<B>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::InvariantF<B> {
        self.map(f)
    }
}

impl<A, E> Functor<'_> for Result<A, E> {
    type FunctorF<B> = Result<B, E>;

    fn map<B>(self, f: impl FnOnce(Self::Domain) -> B) -> Self::FunctorF<B> {
        self.map(f)
    }
}

impl<A, E> Semigroupal for Result<A, E> {
    type From = A;
    type SemigroupalF<B> = Result<B, E>;

    fn product<B>(self, other: Self::SemigroupalF<B>) -> Self::SemigroupalF<(Self::From, B)> {
        match (self, other) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        }
    }
}

impl<'a, F, A, B, E> Apply<'a, A, B> for Result<F, E>
where
    F: FnOnce(A) -> B,
{
    type ApplyF<D> = Result<D, E>;
    fn ap(self, fa: Self::ApplyF<A>) -> Self::ApplyF<B> {
        self.map(|f: F| fa.map(f)).flatten()
    }
}

impl<'a, A, E> InvariantMonoidal<'a> for Result<A, E> {
    fn unit() -> Self::InvariantF<()> {
        Ok(())
    }
}

impl<'a, A, E> Applicative<'a> for Result<A, E> {
    fn pure(x: Self::Domain) -> Self {
        Ok(x)
    }
}

impl<'a, A, E> FlatMap<'a> for Result<A, E> {
    fn flat_map<B>(self, f: impl FnOnce(Self::Domain) -> Self::FunctorF<B>) -> Self::FunctorF<B> {
        match self {
            Ok(x) => f(x),
            Err(e) => Err(e)
        }
    }
} 