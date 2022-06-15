use crate::core::invariant::*;
use crate::kernel::semigroup::*;

impl<'a, Ops, A, State> Invariant<'a> for Semigroup<Ops, A, State>
where
    Ops: Combinable<Domain = A> + 'a,
    State: SemigroupState + 'a,
    A: 'a,
{
    type Domain = A;
    type InvariantF<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>
        // = Semigroup<CombineFn<Box<dyn Fn(B, B) -> B + 'a>, B>, B, Normal>;
        = Semigroup<ComposedCombine<Ops, F, G, B>, B, Normal>;

    fn imap<B: 'a, F: Fn(Self::Domain) -> B + 'a, G: Fn(B) -> Self::Domain + 'a>(
        self,
        f: F,
        g: G,
    ) -> Self::InvariantF<B, F, G> {
        // let ops: Box<dyn Fn(B, B) -> B + '_> = Box::new(move |x, y| f(self.combine(g(x), g(y))));
        // let ops = CombineFn::new(ops);

        // let ops = move |x, y| f(self.combine(g(x), g(y)));
        // let ops = CombineFn::new(ops);

        let ops = ComposedCombine::new(self.ops, f, g);
        Semigroup::new(ops)
    }
}
