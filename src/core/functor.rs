use super::invariant::Invariant;

/**
 * Functor.
 *
 * The name is short for "covariant functor".
 *
 * Must obey the laws defined in cats.laws.FunctorLaws.
 */

pub trait Functor<'a>: Invariant<'a> + Sized {
    type FunctorF<A>;

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
