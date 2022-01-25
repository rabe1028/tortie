use super::{invariant::Invariant, invariant_semigroupal::InvariantSemigroupal};

/**
 * Invariant version of a Monoidal.
 *
 * Must obey the laws defined in cats.laws.InvariantMonoidalLaws.
 */

pub trait InvariantMonoidal<'a>: InvariantSemigroupal<'a> {
    fn point<A: 'a + Clone>(a: A) -> <Self::InvariantF<()> as Invariant<'a>>::InvariantF<A>
    where
        <Self as Invariant<'a>>::InvariantF<()>: Invariant<'a, Domain = ()>,
    {
        <Self as InvariantMonoidal<'a>>::unit().imap(move |_| a.clone(), |_| ())
    }

    fn unit() -> Self::InvariantF<()>;
}
