use super::invariant::Invariant;

/**
 * Functor.
 *
 * The name is short for "covariant functor".
 *
 * Must obey the laws defined in cats.laws.FunctorLaws.
 */

pub trait Functor<'a>: Invariant<'a> + Sized {
    type FunctorF<A>: Functor<'a>;

    fn map<B>(self, f: impl FnOnce(Self::Domain) -> B) -> Self::FunctorF<B>;

    fn imap<B>(
        self,
        f: impl FnOnce(Self::Domain) -> B,
        _: impl FnOnce(B) -> Self::Domain,
    ) -> Self::FunctorF<B> {
        self.map(f)
    }

    ///
    /// Alias for [[map]], since [[map]] can't be injected as syntax if
    /// the implementing type already had a built-in `.map` method.
    ///
    fn fmap<B>(self, f: impl FnOnce(Self::Domain) -> B) -> Self::FunctorF<B> {
        <Self as Functor>::map(self, f)
    }

    /// Cats `as` method but rust cannot use `as` name
    fn replace<B>(self, b: B) -> Self::FunctorF<B> {
        self.map(|_| b)
    }
}

pub trait FunctorLift<'a>: Functor<'a> {
    type Lifted<B>: FnOnce(Self) -> Self::FunctorF<B>;

    fn lift<B, F>(f: F) -> Self::Lifted<B>
    where
        F: FnOnce(Self::Domain) -> B + 'a;
}

impl<'a, A> FunctorLift<'a> for A
where
    A: Functor<'a>,
{
    type Lifted<B> = Box<dyn FnOnce(Self) -> Self::FunctorF<B> + 'a>;

    fn lift<B, F>(f: F) -> Self::Lifted<B>
    where
        F: FnOnce(Self::Domain) -> B + 'a,
    {
        Box::new(move |fa: A| fa.map(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_fmap() {
        let s = Some(1u32);
        assert_eq!(s.fmap(|x| { x as u64 }), Some(1u64))
    }

    #[test]
    fn option_lift() {
        let liftf = Option::lift(|x: u32| x as u64);
        assert_eq!(liftf(Some(1u32)), Some(1u64))
    }
}
