use super::{functor::*, invariant_semigroupal::InvariantSemigroupal};

/**
 * Weaker version of Applicative[F]; has apply but not pure.
 *
 * Must obey the laws defined in cats.laws.ApplyLaws.
 */

pub trait AppliedBound<'a>: FunctorLift<'a> + InvariantSemigroupal<'a> {}

impl<'a, A> AppliedBound<'a> for A where A: FunctorLift<'a> + InvariantSemigroupal<'a> {}

pub trait Apply<'a>: FunctorLift<'a> + InvariantSemigroupal<'a> {
    fn ap<A: 'a + Clone, B: 'a>(self, fa: Self::Rebind<A>) -> Self::Rebind<B>
    where
        Self::Domain: FnMut(A) -> B,
        Self::Rebind<A>: Functor<'a>,
        Self::Rebind<B>: Functor<'a>;

    fn map2<B: 'a, Z: 'a>(
        self,
        fb: Self::Rebind<B>,
        mut f: impl FnMut(Self::Domain, B) -> Z,
    ) -> Self::Rebind<Z>
    where
        Self::Domain: Clone,
        B: Clone,
        Self::Rebind<(Self::Domain, B)>: Functor<'a, Rebind<Z> = Self::Rebind<Z>>,
        Self::Rebind<Z>: Functor<'a>,
    {
        self.product(fb).fmap(|(a, b)| f(a, b))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn option_ap() {
        let a = Some(|x: u32| x + 2);
        assert_eq!(a.ap(Some(1u32)), Some(3u32))
    }

    #[test]
    fn option_map2() {
        assert_eq!(Some(2u32).map2(Some(3u32), |a, b| a + b), Some(5u32));
    }

    #[test]
    fn result_map2() {
        let fa: Result<u32, &str> = Ok(2);
        let fb: Result<u32, &str> = Ok(3);
        assert_eq!(fa.map2(fb, |a, b| a + b), Ok(5u32));
    }

    #[test]
    fn vector_ap_uses_cartesian_product() {
        fn inc(x: u32) -> u32 {
            x + 1
        }

        fn times_ten(x: u32) -> u32 {
            x * 10
        }

        let fs: Vec<fn(u32) -> u32> = vec![inc, times_ten];
        assert_eq!(fs.ap(vec![1, 2]), vec![2, 3, 10, 20]);
    }

    #[test]
    fn vector_map2_uses_cartesian_product() {
        assert_eq!(
            vec![1u32, 2].map2(vec![10u32, 20], |a, b| a + b),
            vec![11, 21, 12, 22]
        );
    }
}
