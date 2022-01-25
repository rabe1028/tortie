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

    fn map<B, F: FnOnce(Self::Domain) -> B>(self, f: F) -> Self::FunctorF<B>
    where
        Self::FunctorF<F>:
            Applicative<'a> + Apply<'a, Self::Domain, B, ApplyF<Self::Domain> = Self>,
        Self::FunctorF<B>: From<<Self::FunctorF<F> as Apply<'a, Self::Domain, B>>::ApplyF<B>>,
    {
        Self::FunctorF::<F>::pure(f).ap(self).into()
    }

    fn unit() -> Self
    where
        Self: Applicative<'a> + Invariant<'a, Domain = ()>,
    {
        Self::pure(())
    }

    fn unless(cond: bool, fa: Self) -> Self::FunctorF<()>
    where
        Self: Applicative<'a> + Invariant<'a> + Functor<'a>,
        <Self as Functor<'a>>::FunctorF<()>: Applicative<'a> + Invariant<'a, Domain = ()>,
    {
        if cond {
            <<Self as Functor<'a>>::FunctorF<()> as Applicative<'a>>::unit()
        } else {
            fa.fmap(|_| ())
        }
    }

    fn when(cond: bool, fa: Self) -> Self::FunctorF<()>
    where
        Self: Applicative<'a> + Invariant<'a> + Functor<'a>,
        Self::FunctorF<()>: Applicative<'a> + Invariant<'a, Domain = ()>,
    {
        if cond {
            fa.fmap(|_| ())
        } else {
            <<Self as Functor<'a>>::FunctorF<()> as Applicative<'a>>::unit()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn option_pure() {
        assert_eq!(Option::pure(1), Some(1));
    }

    #[test]
    fn option_map() {
        assert_eq!(Some(1).map(|x| x * x), Some(1));
    }

    #[test]
    fn option_when() {
        assert_eq!(Option::when(true, Some(1)), Some(()))
    }
}
