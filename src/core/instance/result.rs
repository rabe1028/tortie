use crate::core::invariant::*;

impl<A: Clone, E> Invariant<A> for Result<A, E> {
    type Mapped<'a, B>
    where
        Self: 'a,
    = Result<B, E>;
    fn imap<'a, B: Clone>(
        self,
        f: impl Fn(A) -> B + 'a,
        _g: impl Fn(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: 'a,
    {
        self.map(f)
    }
}
