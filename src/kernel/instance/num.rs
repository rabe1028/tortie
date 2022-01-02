use crate::kernel::semigroup::*;

macro_rules! impl_semigroup_to_num {
    ( $($t:ty), *)  => {
        $(
            impl SemigroupOps<$t> for SemigroupType<$t> {
                type Reversed = SemigroupType<$t, Reversed>;
                fn combine(&self, x: $t, y: $t) -> $t {
                    x + y
                }

                fn reverse(self) -> Self::Reversed {
                    <Self::Reversed as Default>::default()
                }
            }
        )*
    };
}

impl_semigroup_to_num! { u8, u16, u32, u64, u128, usize }
impl_semigroup_to_num! { i8, i16, i32, i64, i128, isize }
