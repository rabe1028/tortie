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
    // type Lifted<A: Clone, B: Clone>: FnOnce(Self::F<A>) -> Self::F<B>;

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

    // fn lift<A: Clone, B: Clone>(f: impl FnOnce(A) -> B) -> Self::Lifted<A, B> {
    //     move |fa: Self::F<A>| <Self as Functor>::map(fa, f)
    // }
}
