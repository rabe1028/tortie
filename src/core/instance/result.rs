use crate::core::invariant::*;

// impl<E> Invariant for ResultHigherKind<E> {
//     fn imap<A, B>(
//         fa: Self::F<A>,
//         f: impl Fn(A) -> B,
//         _g: impl Fn(B) -> A,
//     ) -> Self::F<B> {
//         fa.map(f)
//     }
// }

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
