use crate::core::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal,
};

impl<'a, A: 'a, E: 'a> Invariant<'a> for Result<A, E> {
    type Domain = A;
    type Rebind<B>
        = Result<B, E>
    where
        B: 'a;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::Rebind<B> {
        self.map(|a| f(a))
    }
}

impl<'a, A: 'a, E: 'a> Functor<'a> for Result<A, E> {
    fn map<B: 'a>(self, f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B> {
        self.map(f)
    }
}

impl<'a, A: 'a, E: 'a> Semigroupal<'a> for Result<A, E> {
    fn product<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Domain: Clone,
        B: Clone,
    {
        match (self, other) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        }
    }
}

impl<'a, F, E> Apply<'a> for Result<F, E>
where
    F: 'a,
    E: 'a,
{
    fn ap<A: 'a + Clone, B: 'a>(self, fa: Self::Rebind<A>) -> Self::Rebind<B>
    where
        Self::Domain: FnMut(A) -> B,
        Self::Rebind<A>: Functor<'a>,
        Self::Rebind<B>: Functor<'a>,
    {
        self.map(|f: F| fa.map(f)).flatten()
    }
}

impl<'a, A: 'a, E: 'a> InvariantMonoidal<'a> for Result<A, E> {
    fn unit() -> Self::Rebind<()> {
        Ok(())
    }
}

impl<'a, A: 'a, E: 'a> Applicative<'a> for Result<A, E> {
    fn pure(x: Self::Domain) -> Self {
        Ok(x)
    }
}

impl<'a, A: 'a, E: 'a> FlatMap<'a> for Result<A, E> {
    fn flat_map<B: 'a>(
        self,
        mut f: impl FnMut(Self::Domain) -> Self::Rebind<B>,
    ) -> Self::Rebind<B> {
        match self {
            Ok(x) => f(x),
            Err(e) => Err(e),
        }
    }

    fn tailrec<U: 'a>(a: U, mut f: impl FnMut(U) -> Self::Rebind<Result<Self::Domain, U>>) -> Self {
        // this code happend stack overflow.
        // match f(a) {
        //     Err(e) => Err(e),
        //     Ok(Err(b1)) => Result::tailrec(b1, f),
        //     Ok(Ok(v)) => Ok(v),
        // }
        let mut a = a;
        for _ in 0..Self::TAILREC_LIMIT {
            match f(a) {
                Err(e) => return Err(e),
                Ok(Err(a1)) => a = a1,
                Ok(Ok(v)) => return Ok(v),
            }
        }
        unreachable!("Tailrec limit reached!!!");
    }
}
