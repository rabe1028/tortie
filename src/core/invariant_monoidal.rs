use super::{invariant::Invariant, invariant_semigroupal::InvariantSemigroupal};

/**
 * Invariant version of a Monoidal.
 *
 * Must obey the laws defined in cats.laws.InvariantMonoidalLaws.
 */

pub trait InvariantMonoidal<'a>: InvariantSemigroupal<'a> {
    fn point<A: 'a + Clone>(a: A) -> <Self::Rebind<()> as Invariant<'a>>::Rebind<A>
    where
        <Self as Invariant<'a>>::Rebind<()>: Invariant<'a, Domain = ()>,
    {
        <Self as InvariantMonoidal<'a>>::unit().imap(move |_| a.clone(), |_| ())
    }

    fn unit() -> Self::Rebind<()>;
}
