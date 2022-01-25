use crate::core::invariant::*;
use crate::kernel::semigroup::*;

impl<'a, Ops, A, State> Invariant<'a> for Semigroup<Ops, A, State>
where
    Ops: Combinable<Domain = A> + 'a,
    State: SemigroupState + 'a,
    A: 'a,
{
    type Domain = A;
    type InvariantF<B>
    where
        B: 'a,
    = Semigroup<CombineFn<Box<dyn Fn(B, B) -> B + 'a>, B>, B, Normal>;
    // type InvariantF<B> = Semigroup<CombineFn<impl Fn(B, B) -> B + 'a, B>, B, Normal>;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B + 'a,
        g: impl Fn(B) -> Self::Domain + 'a,
    ) -> Self::InvariantF<B> {
        let ops: Box<dyn Fn(B, B) -> B + '_> = Box::new(move |x, y| f(self.combine(g(x), g(y))));
        let ops = CombineFn::new(ops);

        // let ops = move |x, y| f(self.combine(g(x), g(y)));
        // let ops = CombineFn::new(ops);

        Semigroup::new(ops)
    }
}
