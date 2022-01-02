use super::invariant::Invariant;

/**
 * Functor.
 *
 * The name is short for "covariant functor".
 *
 * Must obey the laws defined in cats.laws.FunctorLaws.
 */

pub trait Functor<A>: Invariant<A> {
    fn map<'a, B>(self, f: impl FnOnce(A) -> B + 'a) -> Self::Mapped<'a, B>;

    fn imap<'a, B>(
        self,
        f: impl FnOnce(A) -> B + 'a,
        _: impl FnOnce(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: Sized,
    {
        self.map(f)
    }
}
