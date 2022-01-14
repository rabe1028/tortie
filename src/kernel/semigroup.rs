use std::marker::PhantomData;



pub trait Combinable {
    type Domain;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain;
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

pub struct Semigroup<Ops, A, State>
where
    Ops: Combinable<Domain = A>,
    State: SemigroupState,
{
    pub ops: Ops,
    _dom: PhantomData<A>,
    _state: PhantomData<State>,
}

impl<Ops, A> Semigroup<Ops, A, Normal>
where
    Ops: Combinable<Domain = A>,
{
    pub fn new(ops: Ops) -> Self {
        Self {
            ops,
            _dom: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Ops, A, State> Semigroup<Ops, A, State>
where
    Ops: Combinable<Domain = A>,
    State: SemigroupState,
{
    pub fn combine(&self, x: A, y: A) -> A {
        self.ops.combine(x, y)
    }

    /// Return `a` combined with itself `n` times.
    pub fn combine_n(&self, a: A, n: usize) -> A
    where
        A: Clone,
    {
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
    pub fn reverse(self) -> Semigroup<Ops, A, State::Reverse> {
        Semigroup {
            ops: self.ops,
            _dom: PhantomData,
            _state: PhantomData,
        }
    }
}

pub struct CombineFn<F, A>
where
    F: Fn(A, A) -> A,
{
    pub f: F,
    _dom: PhantomData<A>,
}

impl<F, A> CombineFn<F, A>
where
    F: Fn(A, A) -> A,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _dom: PhantomData,
        }
    }
}

impl<F, A> Combinable for CombineFn<F, A>
where
    F: Fn(A, A) -> A,
{
    type Domain = A;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        (self.f)(x, y)
    }
}

pub struct StaticCombine<A> {
    _dom: PhantomData<A>,
}

impl<A> Default for StaticCombine<A>
where
    Self: Combinable<Domain = A>,
{
    fn default() -> Self {
        Self { _dom: PhantomData }
    }
}

pub struct ConvertedOps<A, B, Ops, F, G>
where
    Ops: Combinable<Domain = A>,
    F: Fn(A) -> B,
    G: Fn(B) -> A,
{
    base: Ops,
    base2mapped: F,
    mapped2base: G,
    _base_dom: PhantomData<A>,
    _mapped_dom: PhantomData<B>,
}

impl<A, B, Ops, F, G> ConvertedOps<A, B, Ops, F, G>
where
    Ops: Combinable<Domain = A>,
    F: Fn(A) -> B,
    G: Fn(B) -> A,
{
    pub fn new(base: Ops, base2mapped: F, mapped2base: G) -> Self {
        Self {
            base,
            base2mapped,
            mapped2base,
            _base_dom: PhantomData,
            _mapped_dom: PhantomData,
        }
    }
}

impl<A, B, Ops, F, G> Combinable for ConvertedOps<A, B, Ops, F, G>
where
    Ops: Combinable<Domain = A>,
    F: Fn(A) -> B,
    G: Fn(B) -> A,
{
    type Domain = B;
    fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        let (gx, gy) = ((self.mapped2base)(x), (self.mapped2base)(y));
        let go = self.base.combine(gx, gy);
        (self.base2mapped)(go)
    }
}

// impl<A> Combinable for StaticCombine<Option<A>>
// where
//     StaticCombine<A>: Combinable<Domain = A>
// {
//     type Domain = Option<A>;
//     fn combine(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
//         match (x, y) {
//             (None, v) => v,
//             (Some(a), None) => Some(a),
//             (Some(a), Some(b)) => {
//                 let c = StaticCombine::<A>::default();
//                 Some(c.combine(a, b))
//             },
//         }
//     }
// }

pub mod semigroup2 {
    use super::*;

    #[inline]
    pub fn combine<A>(x: A, y: A) -> A
    where
        StaticCombine<A>: Combinable<Domain = A>,
    {
        let f = StaticCombine::<A>::default();
        let semigroup = Semigroup::new(f);
        semigroup.combine(x, y)
    }

    #[inline]
    pub fn combine_n<A: Clone>(a: A, n: usize) -> A
    where
        StaticCombine<A>: Combinable<Domain = A>,
    {
        let f = StaticCombine::<A>::default();
        let semigroup = Semigroup::new(f);
        semigroup.combine_n(a, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::instance::option::*;

    #[test]
    fn combine_string() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup2::combine("Hello ".to_string(), "World!".to_string());
        assert_eq!(answer, "Hello World!".to_string());
    }

    #[test]
    fn combine_option() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = OptionCombine::default().combine(None, Some(1));
        assert_eq!(answer, Some(1))
    }

    #[test]
    fn combine_n_int() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup2::combine_n(1, 10);
        assert_eq!(answer, 10);
    }

    #[test]
    fn combine_n_string() {
        // example from https://github.com/typelevel/cats/blob/main/kernel/src/main/scala/cats/kernel/Semigroup.scala
        let answer = semigroup2::combine_n("ha".to_string(), 3);
        assert_eq!(answer, "hahaha".to_string());
    }

    #[test]
    fn reverse() {
        let a = 1;
        let b = 100;

        let sg = Semigroup::new(StaticCombine::<i32>::default());

        assert_eq!(
            StaticCombine::<i32>::default().combine(a, b),
            sg.reverse().combine(b, a)
        )
    }
}
