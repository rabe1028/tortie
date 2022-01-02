use crate::core::invariant::*;

impl<A: Clone> Invariant<A> for Option<A> {
    type Mapped<'a, B>
    where
        Self: 'a,
    = Option<B>;
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
