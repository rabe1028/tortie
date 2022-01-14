use crate::core::invariant::*;

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
