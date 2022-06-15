#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(result_flattening)]
#![feature(trait_alias)]

use std::{pin::Pin, future::Future, marker::PhantomData};

pub mod core;
pub mod kernel;

trait AsyncTrait {
    type FRet<'async_trait>: Future<Output=()> + Send + 'async_trait;

    fn f<'async_trait>(&self) -> Self::FRet<'async_trait>;
}

struct Runner {}

impl AsyncTrait for Runner {
    type FRet<'async_trait> = impl Future<Output = ()> + Send + 'async_trait;

    fn f<'async_trait>(&self) -> Self::FRet<'async_trait> {
        async move {
            println!("Test")
        }
    }
}

#[cfg(test)]
mod test {
    use futures::executor;

    use crate::*;

    #[test]
    fn test() {
        let runner = Runner {};
        executor::block_on(runner.f());
    }
}
