use super::{
    apply::Apply,
    functor::{Functor, FunctorLift},
    invariant::Invariant,
    invariant_monoidal::InvariantMonoidal,
};

/**
 * Applicative functor.
 *
 * Allows application of a function in an Applicative context to a value in an Applicative context
 *
 * See: [[https://www.cs.ox.ac.uk/jeremy.gibbons/publications/iterator.pdf The Essence of the Iterator Pattern]]
 * Also: [[http://staff.city.ac.uk/~ross/papers/Applicative.pdf Applicative programming with effects]]
 *
 * Must obey the laws defined in cats.laws.ApplicativeLaws.
 */

pub trait Applicative<'a>: FunctorLift<'a> + InvariantMonoidal<'a> {
    fn pure(x: Self::Domain) -> Self;

    fn map<B: 'a, F: FnMut(Self::Domain) -> B + 'a>(self, f: F) -> Self::Rebind<B>
    where
        Self::Domain: Clone,
        Self::Rebind<F>: Applicative<'a, Domain = F, Rebind<Self::Domain> = Self, Rebind<B> = Self::Rebind<B>>
            + Apply<'a>,
        Self::Rebind<B>: Functor<'a>,
    {
        Self::Rebind::<F>::pure(f).ap(self)
    }

    fn unit() -> Self
    where
        Self: Applicative<'a> + Invariant<'a, Domain = ()>,
    {
        Self::pure(())
    }

    fn unless(cond: bool, fa: Self) -> Self::Rebind<()>
    where
        Self: Applicative<'a> + Invariant<'a> + Functor<'a>,
        Self::Rebind<()>: Applicative<'a, Domain = ()> + Invariant<'a, Domain = ()> + Functor<'a>,
    {
        if cond {
            <Self::Rebind<()> as Applicative<'a>>::unit()
        } else {
            fa.fmap(|_| ())
        }
    }

    fn when(cond: bool, fa: Self) -> Self::Rebind<()>
    where
        Self: Applicative<'a> + Invariant<'a> + Functor<'a>,
        Self::Rebind<()>: Applicative<'a, Domain = ()> + Invariant<'a, Domain = ()> + Functor<'a>,
    {
        if cond {
            fa.fmap(|_| ())
        } else {
            <Self::Rebind<()> as Applicative<'a>>::unit()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_pure() {
        assert_eq!(Option::pure(1), Some(1));
    }

    #[test]
    fn option_map() {
        assert_eq!(
            <Option<i32> as Applicative>::map(Some(2), |x| x * x),
            Some(4)
        );
    }

    #[test]
    fn option_when() {
        assert_eq!(Option::when(true, Some(1)), Some(()))
    }

    #[test]
    fn result_pure() {
        let value: Result<u32, &str> = Applicative::pure(1);
        assert_eq!(value, Ok(1));
    }

    #[test]
    fn box_pure_and_map() {
        assert_eq!(*Box::pure(1u32), 1);
        assert_eq!(*<Box<u32> as Applicative>::map(Box::new(2), |x| x * 2), 4);
    }

    #[test]
    fn vector_pure_and_map() {
        assert_eq!(Vec::pure(1u32), vec![1]);
        assert_eq!(
            <Vec<u32> as Applicative>::map(vec![1, 2, 3], |x| x * 2),
            vec![2, 4, 6]
        );
    }
}
