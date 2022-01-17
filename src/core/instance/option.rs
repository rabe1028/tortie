use crate::core::{functor::Functor, invariant::*, semigroupal::Semigroupal};

impl<A> Invariant<'_> for Option<A> {
    type Domain = A;
    type InvariantF<B> = Option<B>;

    fn imap<B>(
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
