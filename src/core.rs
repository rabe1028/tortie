pub mod applicative;
pub mod apply;
pub mod flat_map;
pub mod functor;
pub mod instance;
pub mod invariant;
pub mod invariant_monoidal;
pub mod invariant_semigroupal;
pub mod monad;
pub mod semigroupal;

pub trait Isomorphism<T>: From<T> + Into<T> {}

impl<T, U> Isomorphism<T> for U where U: From<T> + Into<T> {}
