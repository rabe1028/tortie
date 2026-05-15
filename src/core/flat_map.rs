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
    fn flat_map<B: 'a>(self, f: impl FnMut(Self::Domain) -> Self::Rebind<B>) -> Self::Rebind<B>;

    fn flatten(self) -> Self::Rebind<Self::Domain>
    where
        Self::Domain: FlatMap<'a> + Functor<'a, Rebind<Self::Domain> = Self::Domain>,
        Self::Rebind<Self::Domain>: Isomorphism<Self::Domain>,
    {
        self.flat_map(|fa| fa.into())
    }

    fn flat_tap<B: 'a>(
        self,
        mut f: impl FnMut(Self::Domain) -> Self::Rebind<B>,
    ) -> Self::Rebind<Self::Domain>
    where
        Self::Rebind<B>: Functor<'a, Rebind<Self::Domain> = Self::Rebind<Self::Domain>>,
        Self::Rebind<Self::Domain>: Functor<'a>,
        Self::Domain: Clone,
    {
        self.flat_map(|a| f(a.clone()).replace(a))
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
    fn tailrec<U: 'a>(a: U, f: impl FnMut(U) -> Self::Rebind<Result<Self::Domain, U>>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_flat_map() {
        assert_eq!(Some(1).flat_map(|_| Some(2)), Some(2))
    }

    #[test]
    fn result_flat_map() {
        let value: Result<u32, &str> = Ok(1);
        assert_eq!(value.flat_map(|x| Ok(x + 1)), Ok(2));

        let value: Result<u32, &str> = Err("error");
        assert_eq!(value.flat_map(|x| Ok(x + 1)), Err("error"));
    }

    #[test]
    fn box_flat_map() {
        assert_eq!(*Box::new(1u32).flat_map(|x| Box::new(x + 1)), 2);
    }

    #[test]
    fn vector_flat_map() {
        let value = vec![1, 2, 3];
        assert_eq!(
            value.flat_map(|x| vec![x, x * 10]),
            vec![1, 10, 2, 20, 3, 30]
        );
    }

    #[test]
    fn vector_tailrec() {
        assert_eq!(
            Vec::tailrec(0, |x| {
                if x < 3 {
                    vec![Err(x + 1)]
                } else {
                    vec![Ok(x)]
                }
            }),
            vec![3]
        );
    }

    #[test]
    fn option_flatten() {
        assert_eq!(Some(Some(1)).flatten(), Some(1))
    }
}
