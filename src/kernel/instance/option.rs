use crate::kernel::higher_kind::*;
use crate::kernel::semigroup::*;

impl<A> HigherKind for Option<A> {
    type F<B> = Option<B>;
}

impl<A: Clone> SemigroupOps<Option<A>> for SemigroupType<Option<A>>
where
    SemigroupType<A>: SemigroupOps<A>,
{
    type Reversed = SemigroupType<Option<A>, Reversed>;
    fn combine(&self, x: Option<A>, y: Option<A>) -> Option<A> {
        match (x, y) {
            (None, v) => v,
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(semigroup::combine(a, b)),
        }
    }

    fn reverse(self) -> Self::Reversed {
        Self::Reversed::default()
    }
}
