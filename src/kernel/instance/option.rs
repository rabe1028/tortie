use crate::kernel::semigroup::*;
use std::marker::PhantomData;

pub struct OptionCombine<A> {
    _dom: PhantomData<A>,
}

impl<A> Default for OptionCombine<A>
where
    Self: Combinable<Domain = Option<A>>,
{
    fn default() -> Self {
        Self { _dom: PhantomData }
    }
}

impl<A> Combinable for OptionCombine<A>
where
    StaticCombine<A>: Combinable<Domain = A>,
{
    type Domain = Option<A>;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        match (x, y) {
            (None, v) => v,
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => {
                let c = StaticCombine::<A>::default();
                Some(c.combine(a, b))
            }
        }
    }
}
