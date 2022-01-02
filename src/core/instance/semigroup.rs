use crate::core::invariant::*;
use crate::kernel::semigroup::*;

impl<A: Clone, State: SemigroupState> Invariant<A> for SemigroupType<A, State>
where
    Self: SemigroupOps<A, State>,
{
    type Mapped<'a, B>
    where
        Self: 'a,
    = SemigroupInstance<'a, B, Normal>;
    fn imap<'a, B: Clone>(
        self,
        f: impl Fn(A) -> B + 'a,
        g: impl Fn(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: 'a,
    {
        let sg = self;
        let cmb: Box<dyn Fn(B, B) -> B + 'a> = Box::new(move |x, y| f(sg.combine(g(x), g(y))));

        semigroup::from_boxfn(cmb)
    }
}

impl<A: Clone, State: SemigroupState> Invariant<A> for SemigroupInstance<'_, A, State>
where
    Self: SemigroupOps<A, State>,
{
    type Mapped<'a, B>
    where
        Self: 'a,
    = SemigroupInstance<'a, B, Normal>;
    fn imap<'a, B: Clone>(
        self,
        f: impl Fn(A) -> B + 'a,
        g: impl Fn(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: 'a,
    {
        let sg = self;
        let cmb: Box<dyn Fn(B, B) -> B + 'a> = Box::new(move |x, y| f(sg.combine(g(x), g(y))));

        semigroup::from_boxfn(cmb)
    }
}
