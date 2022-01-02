use crate::kernel::semigroup::*;

pub trait Invariant<A> {
    type Mapped<'a, B>
    where
        Self: 'a;

    /// Transform an `F<A>` into an `F<B>` by providing a transformation from
    /// `A` to `B` and one from `B` to `A`
    fn imap<'a, B: Clone>(
        self,
        f: impl Fn(A) -> B + 'a,
        g: impl Fn(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: 'a;
}

impl<A, S> Invariant<A> for S
where
    A: Clone,
    S: SemigroupOps<A>,
{
    type Mapped<'a, B>
    where
        Self: 'a,
    = SemigroupInstance<'a, B, Normal>;
    fn imap<'a, B: Clone>(
        self,
        f: impl Fn(A) -> B + 'a,
        g: impl Fn(B) -> A + 'a,
    ) -> Self::Mapped<'a, B>
    where
        Self: 'a,
    {
        let sg = self;
        let cmb: Box<dyn Fn(B, B) -> B + 'a> = Box::new(move |x, y| f(sg.combine(g(x), g(y))));

        semigroup::from_boxfn(cmb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{
        NaiveDateTime,
        NaiveDate, TimeZone};
    use chrono_tz::EST;

    #[test]
    fn imap_example() {
        // example from https://github.com/typelevel/cats/blob/main/core/src/main/scala/cats/Invariant.scala

        fn i64_to_datetime(secs: i64) -> NaiveDateTime {
            // from unixtimestamp[ms] to datetime
            let (secs, millis) = (secs / 1000, secs % 1000);
            NaiveDateTime::from_timestamp(secs, millis as u32 * 1_000_000) // millis -> nanos
        }

        fn datetime_to_i64(ndt: NaiveDateTime) -> i64 {
            // from datetime to unixtimestamp[ms]
            ndt.timestamp_millis()
        }

        let sg: SemigroupInstance<'_, NaiveDateTime> = Invariant::imap(
            Semigroup::<i64>::default(),
            i64_to_datetime,
            datetime_to_i64,
        );

        let today_ns = 1449088684104;
        let left_ns = 1900918893;
        let today: NaiveDateTime = i64_to_datetime(today_ns);
        let time_left: NaiveDateTime = i64_to_datetime(left_ns);

        let actual = sg.combine(today, time_left);
        assert_eq!(i64_to_datetime(today_ns + left_ns), actual);

        let expect = EST.ymd(2015, 12, 24).and_hms_milli(15, 40, 02, 997);
        assert_eq!(expect.timestamp_millis(), today_ns + left_ns)
    }
}
