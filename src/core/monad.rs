use super::{applicative::Applicative, flat_map::FlatMap, Isomorphism};

/**
 * Monad.
 *
 * Allows composition of dependent effectful functions.
 *
 * See: [[http://homepages.inf.ed.ac.uk/wadler/papers/marktoberdorf/baastad.pdf Monads for functional programming]]
 *
 * Must obey the laws defined in cats.laws.MonadLaws.
 */

pub trait Monad<'a>: FlatMap<'a> + Applicative<'a> {
    /**
     * Execute an action repeatedly until its result fails to satisfy the given predicate
     * and return that result, discarding all others.
     */
    fn iterate_while(self, p: impl Fn(&Self::Domain) -> bool) -> Self
    where
        Self: Clone,
        Self::FunctorF<Self::Domain>: Isomorphism<Self>,
        Self::FunctorF<Result<Self::Domain, Self::Domain>>: Applicative<'a>,
    {
        self.clone()
            .flat_map(|i| Self::iterate_while_m(i, move |_| self.clone(), p).into())
            .into()
    }

    /**
     * Execute an action repeatedly until its result satisfies the given predicate
     * and return that result, discarding all others.
     */
    fn iterate_until(self, p: impl Fn(&Self::Domain) -> bool) -> Self
    where
        Self: Clone,
        Self::FunctorF<Self::Domain>: Isomorphism<Self>,
        Self::FunctorF<Result<Self::Domain, Self::Domain>>: Applicative<'a>,
    {
        self.clone()
            .flat_map(|i| Self::iterate_until_m(i, move |_| self.clone(), p).into())
            .into()
    }

    /**
     * Apply a monadic function iteratively until its result fails
     * to satisfy the given predicate and return that result.
     */
    fn iterate_while_m(
        init: Self::Domain,
        f: impl Fn(Self::Domain) -> Self,
        p: impl Fn(&Self::Domain) -> bool,
    ) -> Self
    where
        Self::FunctorF<Result<Self::Domain, Self::Domain>>: Applicative<'a>,
    {
        Self::tailrec(init, |a| {
            if p(&a) {
                f(a).fmap(|x| Err(x))
            } else {
                Applicative::pure(Ok(a))
            }
        })
    }

    /**
     * Apply a monadic function iteratively until its result satisfies
     * the given predicate and return that result.
     */
    #[inline(always)]
    fn iterate_until_m(
        init: Self::Domain,
        f: impl Fn(Self::Domain) -> Self,
        p: impl Fn(&Self::Domain) -> bool,
    ) -> Self
    where
        Self::FunctorF<Result<Self::Domain, Self::Domain>>: Applicative<'a>,
    {
        Monad::iterate_while_m(init, f, |a| !p(a))
    }
}

impl<'a, T> Monad<'a> for T where T: FlatMap<'a> + Applicative<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_iterate_while() {
        assert_eq!(Some(1).iterate_while(|_| false), Some(1))
    }

    // これは時間かかるので実行しない
    // #[test]
    // #[should_panic]
    // fn option_iterate_until() {
    //     assert_eq!(Some(1).iterate_until(|_| false), None)
    // }
}
