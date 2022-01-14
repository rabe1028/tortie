use crate::kernel::semigroup::*;

macro_rules! impl_semigroup_to_num {
    ( $($t:ty), *)  => {
        $(
            impl Combinable for StaticCombine<$t> {
                type Domain = $t;
                fn combine(&self, x: $t, y: $t) -> $t {
                    x + y
                }
            }
        )*
    };
}

impl_semigroup_to_num! { u8, u16, u32, u64, u128, usize }
impl_semigroup_to_num! { i8, i16, i32, i64, i128, isize }
