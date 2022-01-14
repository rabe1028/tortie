use crate::{core::invariant::*};
// use crate::kernel::semigroup::*;
use crate::kernel::semigroup::*;

// impl<State: SemigroupState> Invariant for SemigroupTypeHigherKind<State> {
//     fn imap<A, B>(
//         fa: SemigroupType<A, State>,
//         f: impl Fn(A) -> B,
//         g: impl Fn(B) -> A,
//     ) -> Self::F<B> {
//         let cmb: Box<dyn Fn(B, B) -> B> = Box::new(move |x, y| f(fa.combine(g(x), g(y))));

//         semigroup::from_boxfn(cmb)
//     }
// }

// impl<'a, State: SemigroupState> Invariant for SemigroupInstanceHigherKind<'a, State>
// where
//     State: 'a
// {
//     type MappedInvariant<A> = SemigroupInstance<'a, A, State>;

//     fn imap<A, B>(
//         fa: Self::MappedInvariant<A>,
//         f: impl Fn(A) -> B,
//         g: impl Fn(B) -> A,
//     ) -> Self::MappedInvariant<B>
//     {
//         let cmb: Box<dyn Fn(B, B) -> B> = Box::new(move |x, y| f(fa.combine(g(x), g(y))));

//         SemigroupInstance::new(cmb)
//     }
// }

impl<'a, Ops, A, State> Invariant<'a> for Semigroup<Ops, A, State>
where
    Ops: Combinable<Domain = A> + 'a,
    State: SemigroupState + 'a,
    A: 'a,
{
    type Domain = A;
    type InvariantF<B> = Semigroup<CombineFn<Box<dyn Fn(B, B) -> B + 'a>, B>, B, Normal>;

    fn imap<B>(
        self,
        f: impl Fn(Self::Domain) -> B + 'a,
        g: impl Fn(B) -> Self::Domain + 'a,
    ) -> Self::InvariantF<B> {
        // let ops: ConvertedOps<_, _, Ops, _, _> = ConvertedOps::new(self.ops, f, g);
        // let ops: Box<dyn Combinable<Domain = B>> = Box::new(ops);
        let ops: Box<dyn Fn(B, B) -> B + '_> = Box::new(move |x, y| f(self.combine(g(x), g(y))));
        let ops = CombineFn::new(ops);

        Semigroup::new(ops)
    }
}
