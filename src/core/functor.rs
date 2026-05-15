use super::invariant::Invariant;

/**
 * Functor.
 *
 * The name is short for "covariant functor".
 *
 * Must obey the laws defined in cats.laws.FunctorLaws.
 */

pub trait Functor<'a>: Invariant<'a> + Sized {
    fn map<B: 'a>(self, f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B>
    where
        Self::Rebind<B>: Functor<'a>;

    fn imap<B>(
        self,
        f: impl FnMut(Self::Domain) -> B,
        _: impl FnMut(B) -> Self::Domain,
    ) -> Self::Rebind<B>
    where
        B: 'a,
        Self::Rebind<B>: Functor<'a>,
    {
        self.map(f)
    }

    ///
    /// Alias for [[map]], since [[map]] can't be injected as syntax if
    /// the implementing type already had a built-in `.map` method.
    ///
    fn fmap<B: 'a>(self, f: impl FnMut(Self::Domain) -> B) -> Self::Rebind<B>
    where
        Self::Rebind<B>: Functor<'a>,
    {
        <Self as Functor>::map(self, f)
    }

    /// Cats `as` method but rust cannot use `as` name
    fn replace<B: 'a + Clone>(self, b: B) -> Self::Rebind<B>
    where
        Self::Rebind<B>: Functor<'a>,
    {
        self.map(|_| b.clone())
    }

    fn void(self) -> Self::Rebind<()>
    where
        Self::Rebind<()>: Functor<'a>,
    {
        self.replace(())
    }

    fn tuple_left<B: 'a + Clone>(self, b: B) -> Self::Rebind<(B, Self::Domain)>
    where
        Self::Rebind<(B, Self::Domain)>: Functor<'a>,
    {
        self.map(|a| (b.clone(), a))
    }

    fn tuple_right<B: 'a + Clone>(self, b: B) -> Self::Rebind<(Self::Domain, B)>
    where
        Self::Rebind<(Self::Domain, B)>: Functor<'a>,
    {
        self.map(|a| (a, b.clone()))
    }
}

pub trait FunctorLift<'a>: Functor<'a> {
    type Lifted<B>: FnOnce(Self) -> Self::Rebind<B>
    where
        B: 'a,
        Self::Rebind<B>: Functor<'a>;

    fn lift<B, F>(f: F) -> Self::Lifted<B>
    where
        B: 'a,
        Self::Rebind<B>: Functor<'a>,
        F: FnMut(Self::Domain) -> B + 'a;
}

impl<'a, A> FunctorLift<'a> for A
where
    A: Functor<'a>,
{
    type Lifted<B>
        = Box<dyn FnOnce(Self) -> Self::Rebind<B> + 'a>
    where
        B: 'a,
        Self::Rebind<B>: Functor<'a>;

    fn lift<B, F>(f: F) -> Self::Lifted<B>
    where
        B: 'a,
        Self::Rebind<B>: Functor<'a>,
        F: FnMut(Self::Domain) -> B + 'a,
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

    #[test]
    fn option_derived_operations() {
        assert_eq!(Some(1u32).void(), Some(()));
        assert_eq!(Some(1u32).tuple_left("left"), Some(("left", 1u32)));
        assert_eq!(Some(1u32).tuple_right("right"), Some((1u32, "right")));
    }
}
