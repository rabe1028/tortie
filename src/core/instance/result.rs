use crate::core::{functor::Functor, invariant::*, semigroupal::Semigroupal};

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
