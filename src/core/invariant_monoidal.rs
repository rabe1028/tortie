use super::{invariant::Invariant, invariant_semigroupal::InvariantSemigroupal, Isomorphism};

/**
 * Invariant version of a Monoidal.
 *
 * Must obey the laws defined in cats.laws.InvariantMonoidalLaws.
 */

pub trait InvariantMonoidal<'a>: InvariantSemigroupal<'a> {
    // type PointF = impl Fn(()) -> Self::Domain;
    // type PointG = impl Fn(Self::Domain) -> ();

    // fn point(a: Self::Domain)
    //  -> Self
    //  //<Self::InvariantF<(), _, _> as Invariant<'a>>::InvariantF<A, F, G>
    // where
    //     Self: Sized,
    //     // <Self::Unit as Invariant<'a>>::InvariantF<Self::Domain, Self::PointF, Self::PointG>: Isomorphism<Self>,
    //     Self: Isomorphism<<Self::Unit as Invariant<'a>>::InvariantF<Self::Domain, Self::PointF, Self::PointG>>,
    //     // Self: Isomorphism<Self::InvariantF<(),F,G>>
    // //     <Self as Invariant<'a>>::InvariantF<()>: Invariant<'a, Domain = ()>,
    // {
    //     Self::unit().imap(move |_| a.clone(), |_| ()).into()
    // }

    type Unit: Invariant<'a, Domain = ()>;
    fn unit() -> Self::Unit;
}
