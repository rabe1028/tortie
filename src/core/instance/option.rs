use crate::core::invariant::*;

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
