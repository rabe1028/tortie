use super::{apply::AppliedBound, functor::Functor, Isomorphism};

/**
 * FlatMap type class gives us flatMap, which allows us to have a value
 * in a context (F[A]) and then feed that into a function that takes
 * a normal value and returns a value in a context (A => F[B]).
 *
 * One motivation for separating this out from Monad is that there are
 * situations where we can implement flatMap but not pure.  For example,
 * we can implement map or flatMap that transforms the values of Map[K, *],
 * but we can't implement pure (because we wouldn't know what key to use
 * when instantiating the new Map).
 *
 * @see See [[https://github.com/typelevel/cats/issues/3]] for some discussion.
 *
 * Must obey the laws defined in cats.laws.FlatMapLaws.
 */

pub trait FlatMap<'a>: AppliedBound<'a> {
    fn flat_map<B>(self, f: impl FnOnce(Self::Domain) -> Self::FunctorF<B>) -> Self::FunctorF<B>;

    fn flatten(self) -> Self::FunctorF<Self::Domain>
    where
        Self::Domain: FlatMap<'a> + Functor<'a, FunctorF<Self::Domain> = Self::Domain>,
        Self::FunctorF<Self::Domain>: Isomorphism<Self::Domain>,
    {
        self.flat_map(|fa| fa.into())
    }

    fn flat_tap<B>(self, f: impl FnOnce(Self::Domain) -> Self::FunctorF<B>) -> Self
    where
        Self::FunctorF<Self::Domain>: Isomorphism<Self>
            + Isomorphism<<Self::FunctorF<B> as Functor<'a>>::FunctorF<Self::Domain>>,
        Self::Domain: Clone,
        Self: From<Self::FunctorF<Self::Domain>>,
    {
        self.flat_map(|a| f(a.clone()).replace(a).into()).into()
    }

    /**
     * Keeps calling `f` until a `scala.util.Right[B]` is returned.
     *
     * Based on Phil Freeman's
     * [[http://functorial.com/stack-safety-for-free/index.pdf Stack Safety for Free]].
     *
     * Implementations of this method should use constant stack space relative to `f`.
     */
    const TAILREC_LIMIT: usize = 1_000_000_000;
    fn tailrec<U>(a: U, f: impl Fn(U) -> Self::FunctorF<Result<Self::Domain, U>>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_flat_map() {
        assert_eq!(Some(1).flat_map(|_| Some(2)), Some(2))
    }

    #[test]
    fn option_flatten() {
        assert_eq!(Some(Some(1)).flatten(), Some(1))
    }
}
