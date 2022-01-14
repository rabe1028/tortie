use crate::kernel::semigroup::*;
use std::marker::PhantomData;

pub struct ResultCombine<A, E> {
    _dom: PhantomData<(A, E)>,
}

impl<A, E> Default for ResultCombine<A, E>
where
    Self: Combinable<Domain = Result<A, E>>,
{
    fn default() -> Self {
        Self { _dom: PhantomData }
    }
}

impl<A, E> Combinable for ResultCombine<A, E>
where
    StaticCombine<A>: Combinable<Domain = A>,
{
    type Domain = Result<A, E>;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        match (x, y) {
            (Err(_), v) => v,
            (Ok(a), Err(_)) => Ok(a),
            (Ok(a), Ok(b)) => {
                let c = StaticCombine::<A>::default();
                Ok(c.combine(a, b))
            },
        }
    }
}
