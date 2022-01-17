use super::{invariant::Invariant, semigroupal::Semigroupal};

/**
 * [[InvariantSemigroupal]] is nothing more than something both invariant
 * and Semigroupal. It comes up enough to be useful, and composes well
 */

pub trait InvariantSemigroupal<'a>: Semigroupal + Invariant<'a> {}

impl<'a, A> InvariantSemigroupal<'a> for A where A: Semigroupal + Invariant<'a> {}
