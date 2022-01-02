use std::marker::PhantomData;

use super::higher_kind::HigherKind;

/// A semigroup is any set `A` with an associative operation (`combine`).
pub type Semigroup<A> = SemigroupType<A>;

pub mod semigroup {
    use super::*;

    #[inline]
    pub fn combine<A: Clone>(x: A, y: A) -> A
    where
        SemigroupType<A>: SemigroupOps<A>,
    {
        let f: SemigroupType<A> = SemigroupType::<A>::default();
        f.combine(x, y)
    }

    #[inline]
    pub fn combine_n<A: Clone>(a: A, n: usize) -> A
    where
        SemigroupType<A>: SemigroupOps<A>,
    {
        let f: SemigroupType<A> = SemigroupType::<A>::default();
        f.combine_n(a, n)
    }

    pub fn from_boxfn<'a, A: Clone>(f: Box<dyn Fn(A, A) -> A + 'a>) -> SemigroupInstance<'a, A> {
        SemigroupInstance::from_boxfn(f)
    }
}

pub trait SemigroupOps<A: Clone, State: SemigroupState = Normal>: HigherKind {
    type Reversed: SemigroupOps<A, State::Reverse>;

    fn combine(&self, x: A, y: A) -> A;

    /// Return `a` combined with itself `n` times.
    fn combine_n(&self, a: A, n: usize) -> A {
        if n == 0 {
            panic!("Repeated combining for semigroups must have n > 0")
        } else if n == 1 {
            a
        } else {
            let mut b = a.clone();
            let mut k = n - 1;
            let mut extra = a;
            loop {
                if k == 1 {
                    return self.combine(b, extra);
                } else {
                    let x = if k & 1 == 1 {
                        self.combine(b.clone(), extra)
                    } else {
                        extra
                    };

                    b = self.combine(b.clone(), b.clone());
                    extra = x;
                    k = k >> 1;
                }
            }
        }
    }

    /// return a semigroup that reverses the order
    /// so combine(a, b) == reverse.combine(b, a)
    fn reverse(self) -> Self::Reversed;
}

pub trait SemigroupState {
    type Reverse: SemigroupState;
}
pub enum Normal {}
impl SemigroupState for Normal {
    type Reverse = Reversed;
}
pub enum Reversed {}
impl SemigroupState for Reversed {
    type Reverse = Normal;
}

pub struct SemigroupType<A, State = Normal>
where
    State: SemigroupState,
{
    _phantom: PhantomData<A>,
    _state: PhantomData<State>,
}

impl<A, State> Default for SemigroupType<A, State>
where
    State: SemigroupState,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<A, State> HigherKind for SemigroupType<A, State>
where
    State: SemigroupState,
{
    type F<B> = SemigroupType<B, State>;
}

impl<A: Clone> SemigroupOps<A, Reversed> for SemigroupType<A, Reversed>
where
    SemigroupType<A>: SemigroupOps<A>,
{
    type Reversed = SemigroupType<A, Normal>;

    fn combine(&self, x: A, y: A) -> A {
        let f = Self::Reversed::default();
        f.combine(y, x)
    }

    fn reverse(self) -> Self::Reversed {
        <Self::Reversed as Default>::default()
    }
}

// create instant semigroup from functimn
pub struct SemigroupInstance<'a, A, State = Normal> {
    cmb: Box<dyn Fn(A, A) -> A + 'a>,
    _phantom: PhantomData<A>,
    _state: PhantomData<State>,
}

impl<'a, A> SemigroupInstance<'a, A, Normal>
where
    A: Clone,
{
    pub fn from_boxfn(f: Box<dyn Fn(A, A) -> A + 'a>) -> SemigroupInstance<'a, A, Normal> {
        SemigroupInstance {
            cmb: f,
            _phantom: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<'a, A: Clone, State> HigherKind for SemigroupInstance<'a, A, State>
where
    State: SemigroupState,
{
    type F<B> = SemigroupInstance<'a, B, State>;
}

impl<'a, A: Clone> SemigroupOps<A, Normal> for SemigroupInstance<'a, A, Normal> {
    type Reversed = SemigroupInstance<'a, A, Reversed>;
    fn combine(&self, x: A, y: A) -> A {
        (self.cmb)(x, y)
    }

    fn reverse(self) -> Self::Reversed {
        SemigroupInstance {
            cmb: self.cmb,
            _phantom: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<'a, A: Clone> SemigroupOps<A, Reversed> for SemigroupInstance<'a, A, Reversed> {
    type Reversed = SemigroupInstance<'a, A, Normal>;
    fn combine(&self, x: A, y: A) -> A {
        (self.cmb)(y, x)
    }

    fn reverse(self) -> Self::Reversed {
        SemigroupInstance {
            cmb: self.cmb,
            _phantom: PhantomData,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_string() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup::combine("Hello ".to_string(), "World!".to_string());
        assert_eq!(answer, "Hello World!".to_string());
    }

    #[test]
    fn combine_option() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup::combine(None, Some(1));
        assert_eq!(answer, Some(1))
    }

    #[test]
    fn combine_n_int() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup::combine_n(1, 10);
        assert_eq!(answer, 10);
    }

    #[test]
    fn combine_n_string() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup::combine_n("ha".to_string(), 3);
        assert_eq!(answer, "hahaha".to_string());
    }

    #[test]
    fn reverse() {
        let a = 1;
        let b = 100;
        assert_eq!(
            semigroup::combine(a, b),
            Semigroup::<i32>::default().reverse().combine(b, a)
        )
    }
}
