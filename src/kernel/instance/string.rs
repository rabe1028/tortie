use crate::kernel::semigroup::*;

impl SemigroupOps<String> for SemigroupType<String> {
    type Reversed = SemigroupType<String, Reversed>;
    fn combine(&self, x: String, y: String) -> String {
        x + &y
    }

    fn reverse(self) -> Self::Reversed {
        Self::Reversed::default()
    }
}
