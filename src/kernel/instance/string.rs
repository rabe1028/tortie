use crate::kernel::semigroup::*;

impl Combinable for StaticCombine<String> {
    type Domain = String;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        x + &y
    }
}
