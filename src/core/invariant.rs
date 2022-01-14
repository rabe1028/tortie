

pub trait Invariant<'a> {
    type Domain;
    type InvariantF<A>;

    /// Transform an `F<A>` into an `F<B>` by providing a transformation from
    /// `A` to `B` and one from `B` to `A`
    fn imap<B>(
        self,
        f: impl Fn(Self::Domain) -> B + 'a,
        g: impl Fn(B) -> Self::Domain + 'a,
    ) -> Self::InvariantF<B>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::semigroup::*;
    use chrono::{NaiveDateTime, TimeZone};
    use chrono_tz::*;

    #[test]
    fn option_test() {
        let s = Some(1u32);
        assert_eq!(
            Option::imap::<u64>(s, |x: u32| { x as u64 }, |_: u64| unimplemented!()),
            Some(1u64)
        );
    }

    #[test]
    fn imap_example() {
        // example from https://typelevel.org/cats/typeclasses/invariant.html

        fn i64_to_datetime(secs: i64) -> NaiveDateTime {
            // from unixtimestamp[ms] to datetime
            let (secs, millis) = (secs / 1000, secs % 1000);
            NaiveDateTime::from_timestamp(secs, millis as u32 * 1_000_000) // millis -> nanos
        }

        fn datetime_to_i64(ndt: NaiveDateTime) -> i64 {
            // from datetime to unixtimestamp[ms]
            ndt.timestamp_millis()
        }

        // let sg: SemigroupInstance<'_, NaiveDateTime> = Semigroup::imap::<NaiveDateTime>(
        //     Semigrouped::<i64>::default().into_instance(),
        //     i64_to_datetime,
        //     datetime_to_i64,
        // );

        let sg = Semigroup::new(StaticCombine::<i64>::default());
        let sg = sg.imap(i64_to_datetime, datetime_to_i64);

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
