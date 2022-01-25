use super::{functor::*, invariant_semigroupal::InvariantSemigroupal};

/**
 * Weaker version of Applicative[F]; has apply but not pure.
 *
 * Must obey the laws defined in cats.laws.ApplyLaws.
 */

pub trait AppliedBound<'a> = FunctorLift<'a> + InvariantSemigroupal<'a>;

pub trait Apply<'a, A, B>: FunctorLift<'a> + InvariantSemigroupal<'a> {
    type ApplyF<D>;
    fn ap(self, fa: Self::ApplyF<A>) -> Self::ApplyF<B>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn option_ap() {
        let a = Some(|x: u32| x + 2);
        assert_eq!(a.ap(Some(1u32)), Some(3u32))
    }
}
