use super::invariant::Invariant;

/**
 * [[Semigroupal]] captures the idea of composing independent effectful values.
 * It is of particular interest when taken together with [[Functor]] - where [[Functor]]
 * captures the idea of applying a unary pure function to an effectful value,
 * calling `product` with `map` allows one to apply a function of arbitrary arity to multiple
 * independent effectful values.
 *
 * That same idea is also manifested in the form of [[Apply]], and indeed [[Apply]] extends both
 * [[Semigroupal]] and [[Functor]] to illustrate this.
 */

pub trait Semigroupal<'a>: Invariant<'a> {
    fn product<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Domain: Clone,
        B: Clone;

    fn product_left<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<Self::Domain>
    where
        Self: Sized,
        Self::Domain: Clone,
        B: Clone,
        Self::Rebind<(Self::Domain, B)>:
            crate::core::functor::Functor<'a, Rebind<Self::Domain> = Self::Rebind<Self::Domain>>,
        Self::Rebind<Self::Domain>: crate::core::functor::Functor<'a>,
    {
        crate::core::functor::Functor::map(self.product(other), |(a, _)| a)
    }

    fn product_right<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<B>
    where
        Self: Sized,
        Self::Domain: Clone,
        B: Clone,
        Self::Rebind<(Self::Domain, B)>:
            crate::core::functor::Functor<'a, Rebind<B> = Self::Rebind<B>>,
        Self::Rebind<B>: crate::core::functor::Functor<'a>,
    {
        crate::core::functor::Functor::map(self.product(other), |(_, b)| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_product_operations() {
        assert_eq!(Some(1u32).product(Some("x")), Some((1u32, "x")));
        assert_eq!(Some(1u32).product_left(Some("x")), Some(1u32));
        assert_eq!(Some(1u32).product_right(Some("x")), Some("x"));
    }

    #[test]
    fn result_product_operations() {
        let fa: Result<u32, &str> = Ok(1);
        let fb: Result<&str, &str> = Ok("x");
        assert_eq!(fa.product(fb), Ok((1u32, "x")));
    }

    #[test]
    fn vector_product_operations_use_cartesian_product() {
        assert_eq!(
            vec![1u32, 2].product(vec!["x", "y"]),
            vec![(1u32, "x"), (1, "y"), (2, "x"), (2, "y")]
        );
    }
}
