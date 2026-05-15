use crate::core::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, invariant::*,
    invariant_monoidal::InvariantMonoidal, semigroupal::Semigroupal,
};

impl<'a, A: 'a> Invariant<'a> for Vec<A> {
    type Domain = A;
    type Rebind<B>
        = Vec<B>
    where
        B: 'a;

    fn imap<B: 'a>(
        self,
        f: impl Fn(Self::Domain) -> B,
        _: impl Fn(B) -> Self::Domain,
    ) -> Self::Rebind<B> {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A: 'a> Functor<'a> for Vec<A> {
    fn map<B: 'a>(self, f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B> {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A: 'a> Semigroupal<'a> for Vec<A> {
    fn product<B: 'a>(self, other: Self::Rebind<B>) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Domain: Clone,
        B: Clone,
    {
        self.into_iter()
            .flat_map(|a| other.iter().cloned().map(move |b| (a.clone(), b)))
            .collect()
    }
}

impl<'a, F: 'a> Apply<'a> for Vec<F> {
    fn ap<A: 'a + Clone, B: 'a>(self, fa: Self::Rebind<A>) -> Self::Rebind<B>
    where
        Self::Domain: FnMut(A) -> B,
        Self::Rebind<A>: Functor<'a>,
        Self::Rebind<B>: Functor<'a>,
    {
        self.into_iter()
            .flat_map(|mut f| fa.iter().cloned().map(move |a| f(a)))
            .collect()
    }
}

impl<'a, A: 'a> InvariantMonoidal<'a> for Vec<A> {
    fn unit() -> Self::Rebind<()> {
        vec![()]
    }
}

impl<'a, A: 'a> Applicative<'a> for Vec<A> {
    fn pure(x: Self::Domain) -> Self {
        vec![x]
    }
}

impl<'a, A: 'a> FlatMap<'a> for Vec<A> {
    fn flat_map<B: 'a>(self, f: impl FnMut(Self::Domain) -> Self::Rebind<B>) -> Self::Rebind<B> {
        self.into_iter().flat_map(f).collect()
    }

    fn tailrec<U: 'a>(a: U, mut f: impl FnMut(U) -> Self::Rebind<Result<Self::Domain, U>>) -> Self {
        let mut pending = std::collections::VecDeque::from([a]);
        let mut values = Vec::new();

        while let Some(next) = pending.pop_front() {
            for item in f(next) {
                match item {
                    Ok(value) => values.push(value),
                    Err(next) => pending.push_back(next),
                }
            }
        }

        values
    }
}
