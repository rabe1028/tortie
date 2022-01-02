use crate::kernel::higher_kind::*;

impl<U, E> HigherKind for Result<U, E> {
    type F<B> = Result<B, E>;
}
