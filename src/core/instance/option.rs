use crate::core::invariant::*;

// impl Invariant for OptionHigherKind {
//     fn imap<A, B>(
//         fa: Self::F<A>,
//         f: impl Fn(A) -> B,
//         _g: impl Fn(B) -> A,
//     ) -> Self::F<B> {
//         fa.map(f)
//     }
// }

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
