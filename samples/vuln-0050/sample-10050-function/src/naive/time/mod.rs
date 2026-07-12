// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! ISO 8601 time without timezone.

use core::fmt;

use num_integer::div_mod_floor;

#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::oldtime::Duration as OldDuration;
use crate::Timelike;

/// ISO 8601 time without timezone.
/// Allows for the nanosecond precision and optional leap second representation.
///
/// # Leap Second Handling
///
/// Since 1960s, the manmade atomic clock has been so accurate that
/// it is much more accurate than Earth's own motion.
/// It became desirable to define the civil time in terms of the atomic clock,
/// but that risks the desynchronization of the civil time from Earth.
/// To account for this, the designers of the Coordinated Universal Time (UTC)
/// made that the UTC should be kept within 0.9 seconds of the observed Earth-bound time.
/// When the mean solar day is longer than the ideal (86,400 seconds),
/// the error slowly accumulates and it is necessary to add a **leap second**
/// to slow the UTC down a bit.
/// (We may also remove a second to speed the UTC up a bit, but it never happened.)
/// The leap second, if any, follows 23:59:59 of June 30 or December 31 in the UTC.
///
/// Fast forward to the 21st century,
/// we have seen 26 leap seconds from January 1972 to December 2015.
/// Yes, 26 seconds. Probably you can read this paragraph within 26 seconds.
/// But those 26 seconds, and possibly more in the future, are never predictable,
/// and whether to add a leap second or not is known only before 6 months.
/// Internet-based clocks (via NTP) do account for known leap seconds,
/// but the system API normally doesn't (and often can't, with no network connection)
/// and there is no reliable way to retrieve leap second information.
///
/// Chrono does not try to accurately implement leap seconds; it is impossible.
/// Rather, **it allows for leap seconds but behaves as if there are *no other* leap seconds.**
/// Various operations will ignore any possible leap second(s)
/// except when any of the operands were actually leap seconds.
///
/// If you cannot tolerate this behavior,
/// you must use a separate `TimeZone` for the International Atomic Time (TAI).
/// TAI is like UTC but has no leap seconds, and thus slightly differs from UTC.
/// Chrono does not yet provide such implementation, but it is planned.
///
/// ## Representing Leap Seconds
///
/// The leap second is indicated via fractional seconds more than 1 second.
/// This makes possible to treat a leap second as the prior non-leap second
/// if you don't care about sub-second accuracy.
/// You should use the proper formatting to get the raw leap second.
///
/// All methods accepting fractional seconds will accept such values.
///
/// ```
/// use chrono::{NaiveDate, NaiveTime, Utc, TimeZone};
///
/// let t = NaiveTime::from_hms_milli(8, 59, 59, 1_000);
///
/// let dt1 = NaiveDate::from_ymd(2015, 7, 1).and_hms_micro(8, 59, 59, 1_000_000);
///
/// let dt2 = Utc.ymd(2015, 6, 30).and_hms_nano(23, 59, 59, 1_000_000_000);
/// # let _ = (t, dt1, dt2);
/// ```
///
/// Note that the leap second can happen anytime given an appropriate time zone;
/// 2015-07-01 01:23:60 would be a proper leap second if UTC+01:24 had existed.
/// Practically speaking, though, by the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.
///
/// ## Date And Time Arithmetics
///
/// As a concrete example, let's assume that `03:00:60` and `04:00:60` are leap seconds.
/// In reality, of course, leap seconds are separated by at least 6 months.
/// We will also use some intuitive concise notations for the explanation.
///
/// `Time + Duration`
/// (short for [`NaiveTime::overflowing_add_signed`](#method.overflowing_add_signed)):
///
/// - `03:00:00 + 1s = 03:00:01`.
/// - `03:00:59 + 60s = 03:02:00`.
/// - `03:00:59 + 1s = 03:01:00`.
/// - `03:00:60 + 1s = 03:01:00`.
///   Note that the sum is identical to the previous.
/// - `03:00:60 + 60s = 03:01:59`.
/// - `03:00:60 + 61s = 03:02:00`.
/// - `03:00:60.1 + 0.8s = 03:00:60.9`.
///
/// `Time - Duration`
/// (short for [`NaiveTime::overflowing_sub_signed`](#method.overflowing_sub_signed)):
///
/// - `03:00:00 - 1s = 02:59:59`.
/// - `03:01:00 - 1s = 03:00:59`.
/// - `03:01:00 - 60s = 03:00:00`.
/// - `03:00:60 - 60s = 03:00:00`.
///   Note that the result is identical to the previous.
/// - `03:00:60.7 - 0.4s = 03:00:60.3`.
/// - `03:00:60.7 - 0.9s = 03:00:59.8`.
///
/// `Time - Time`
/// (short for [`NaiveTime::signed_duration_since`](#method.signed_duration_since)):
///
/// - `04:00:00 - 03:00:00 = 3600s`.
/// - `03:01:00 - 03:00:00 = 60s`.
/// - `03:00:60 - 03:00:00 = 60s`.
///   Note that the difference is identical to the previous.
/// - `03:00:60.6 - 03:00:59.4 = 1.2s`.
/// - `03:01:00 - 03:00:59.8 = 0.2s`.
/// - `03:01:00 - 03:00:60.5 = 0.5s`.
///   Note that the difference is larger than the previous,
///   even though the leap second clearly follows the previous whole second.
/// - `04:00:60.9 - 03:00:60.1 =
///   (04:00:60.9 - 04:00:00) + (04:00:00 - 03:01:00) + (03:01:00 - 03:00:60.1) =
///   60.9s + 3540s + 0.9s = 3601.8s`.
///
/// In general,
///
/// - `Time + Duration` unconditionally equals to `Duration + Time`.
///
/// - `Time - Duration` unconditionally equals to `Time + (-Duration)`.
///
/// - `Time1 - Time2` unconditionally equals to `-(Time2 - Time1)`.
///
/// - Associativity does not generally hold, because
///   `(Time + Duration1) - Duration2` no longer equals to `Time + (Duration1 - Duration2)`
///   for two positive durations.
///
///     - As a special case, `(Time + Duration) - Duration` also does not equal to `Time`.
///
///     - If you can assume that all durations have the same sign, however,
///       then the associativity holds:
///       `(Time + Duration1) + Duration2` equals to `Time + (Duration1 + Duration2)`
///       for two positive durations.
///
/// ## Reading And Writing Leap Seconds
///
/// The "typical" leap seconds on the minute boundary are
/// correctly handled both in the formatting and parsing.
/// The leap second in the human-readable representation
/// will be represented as the second part being 60, as required by ISO 8601.
///
/// ```
/// use chrono::{Utc, TimeZone};
///
/// let dt = Utc.ymd(2015, 6, 30).and_hms_milli(23, 59, 59, 1_000);
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60Z");
/// ```
///
/// There are hypothetical leap seconds not on the minute boundary
/// nevertheless supported by Chrono.
/// They are allowed for the sake of completeness and consistency;
/// there were several "exotic" time zone offsets with fractional minutes prior to UTC after all.
/// For such cases the human-readable representation is ambiguous
/// and would be read back to the next non-leap second.
///
/// ```
/// use chrono::{DateTime, Utc, TimeZone};
///
/// let dt = Utc.ymd(2015, 6, 30).and_hms_milli(23, 56, 4, 1_000);
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:56:05Z");
///
/// let dt = Utc.ymd(2015, 6, 30).and_hms(23, 56, 5);
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:56:05Z");
/// assert_eq!(DateTime::parse_from_rfc3339("2015-06-30T23:56:05Z").unwrap(), dt);
/// ```
///
/// Since Chrono alone cannot determine any existence of leap seconds,
/// **there is absolutely no guarantee that the leap second read has actually happened**.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct NaiveTime {
    secs: u32,
    frac: u32,
}

impl NaiveTime {
    /// Makes a new `NaiveTime` from hour, minute and second.
    ///
    /// No [leap second](#leap-second-handling) is allowed here;
    /// use `NaiveTime::from_hms_*` methods with a subsecond parameter instead.
    ///
    /// Panics on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let t = NaiveTime::from_hms(23, 56, 4);
    /// assert_eq!(t.hour(), 23);
    /// assert_eq!(t.minute(), 56);
    /// assert_eq!(t.second(), 4);
    /// assert_eq!(t.nanosecond(), 0);
    /// ```
    #[inline]
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, min, sec).expect("invalid time")
    }

    /// Makes a new `NaiveTime` from hour, minute and second.
    ///
    /// No [leap second](#leap-second-handling) is allowed here;
    /// use `NaiveTime::from_hms_*_opt` methods with a subsecond parameter instead.
    ///
    /// Returns `None` on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hms_opt = NaiveTime::from_hms_opt;
    ///
    /// assert!(from_hms_opt(0, 0, 0).is_some());
    /// assert!(from_hms_opt(23, 59, 59).is_some());
    /// assert!(from_hms_opt(24, 0, 0).is_none());
    /// assert!(from_hms_opt(23, 60, 0).is_none());
    /// assert!(from_hms_opt(23, 59, 60).is_none());
    /// ```
    #[inline]
    pub fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, 0)
    }

    /// Makes a new `NaiveTime` from hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let t = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(t.hour(), 23);
    /// assert_eq!(t.minute(), 56);
    /// assert_eq!(t.second(), 4);
    /// assert_eq!(t.nanosecond(), 12_345_678);
    /// ```
    #[inline]
    pub fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nano).expect("invalid time")
    }

    /// Makes a new `NaiveTime` from hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hmsn_opt = NaiveTime::from_hms_nano_opt;
    ///
    /// assert!(from_hmsn_opt(0, 0, 0, 0).is_some());
    /// assert!(from_hmsn_opt(23, 59, 59, 999_999_999).is_some());
    /// assert!(from_hmsn_opt(23, 59, 59, 1_999_999_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_hmsn_opt(24, 0, 0, 0).is_none());
    /// assert!(from_hmsn_opt(23, 60, 0, 0).is_none());
    /// assert!(from_hmsn_opt(23, 59, 60, 0).is_none());
    /// assert!(from_hmsn_opt(23, 59, 59, 2_000_000_000).is_none());
    /// ```
    #[inline]
    pub fn from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveTime> {
        if hour >= 24 || min >= 60 || sec >= 60 || nano >= 2_000_000_000 {
            return None;
        }
        let secs = hour * 3600 + min * 60 + sec;
        Some(NaiveTime { secs, frac: nano })
    }

    /// Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid number of seconds and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_nsecs_opt = NaiveTime::from_num_seconds_from_midnight_opt;
    ///
    /// assert!(from_nsecs_opt(0, 0).is_some());
    /// assert!(from_nsecs_opt(86399, 999_999_999).is_some());
    /// assert!(from_nsecs_opt(86399, 1_999_999_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_nsecs_opt(86_400, 0).is_none());
    /// assert!(from_nsecs_opt(86399, 2_000_000_000).is_none());
    /// ```
    #[inline]
    pub fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<NaiveTime> {
        if secs >= 86_400 || nano >= 2_000_000_000 {
            return None;
        }
        Some(NaiveTime { secs, frac: nano })
    }

    /// Adds given `Duration` to the current time,
    /// and also returns the number of *seconds*
    /// in the integral number of days ignored from the addition.
    /// (We cannot return `Duration` because it is subject to overflow or underflow.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Duration, NaiveTime};
    ///
    /// let from_hms = NaiveTime::from_hms;
    ///
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(Duration::hours(11)),
    ///            (from_hms(14, 4, 5), 0));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(Duration::hours(23)),
    ///            (from_hms(2, 4, 5), 86_400));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(Duration::hours(-7)),
    ///            (from_hms(20, 4, 5), -86_400));
    /// ```
    #[cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
    pub fn overflowing_add_signed(&self, mut rhs: OldDuration) -> (NaiveTime, i64) {
        let mut secs = self.secs;
        let mut frac = self.frac;

        // check if `self` is a leap second and adding `rhs` would escape that leap second.
        // if it's the case, update `self` and `rhs` to involve no leap second;
        // otherwise the addition immediately finishes.
        if frac >= 1_000_000_000 {
            let rfrac = 2_000_000_000 - frac;
            if rhs >= OldDuration::nanoseconds(i64::from(rfrac)) {
                rhs = rhs - OldDuration::nanoseconds(i64::from(rfrac));
                secs += 1;
                frac = 0;
            } else if rhs < OldDuration::nanoseconds(-i64::from(frac)) {
                rhs = rhs + OldDuration::nanoseconds(i64::from(frac));
                frac = 0;
            } else {
                frac = (i64::from(frac) + rhs.num_nanoseconds().unwrap()) as u32;
                debug_assert!(frac < 2_000_000_000);
                return (NaiveTime { secs, frac }, 0);
            }
        }
        debug_assert!(secs <= 86_400);
        debug_assert!(frac < 1_000_000_000);

        let rhssecs = rhs.num_seconds();
        let rhsfrac = (rhs - OldDuration::seconds(rhssecs))
            .num_nanoseconds()
            .unwrap();
        debug_assert_eq!(
            OldDuration::seconds(rhssecs) + OldDuration::nanoseconds(rhsfrac),
            rhs
        );
        let rhssecsinday = rhssecs % 86_400;
        let mut morerhssecs = rhssecs - rhssecsinday;
        let rhssecs = rhssecsinday as i32;
        let rhsfrac = rhsfrac as i32;
        debug_assert!(-86_400 < rhssecs && rhssecs < 86_400);
        debug_assert_eq!(morerhssecs % 86_400, 0);
        debug_assert!(-1_000_000_000 < rhsfrac && rhsfrac < 1_000_000_000);

        let mut secs = secs as i32 + rhssecs;
        let mut frac = frac as i32 + rhsfrac;
        debug_assert!(-86_400 < secs && secs < 2 * 86_400);
        debug_assert!(-1_000_000_000 < frac && frac < 2_000_000_000);

        if frac < 0 {
            frac += 1_000_000_000;
            secs -= 1;
        } else if frac >= 1_000_000_000 {
            frac -= 1_000_000_000;
            secs += 1;
        }
        debug_assert!(-86_400 <= secs && secs < 2 * 86_400);
        debug_assert!(0 <= frac && frac < 1_000_000_000);

        if secs < 0 {
            secs += 86_400;
            morerhssecs -= 86_400;
        } else if secs >= 86_400 {
            secs -= 86_400;
            morerhssecs += 86_400;
        }
        debug_assert!(0 <= secs && secs < 86_400);

        (
            NaiveTime {
                secs: secs as u32,
                frac: frac as u32,
            },
            morerhssecs,
        )
    }

    /// Returns a triple of the hour, minute and second numbers.
    fn hms(&self) -> (u32, u32, u32) {
        let (mins, sec) = div_mod_floor(self.secs, 60);
        let (hour, min) = div_mod_floor(mins, 60);
        (hour, min, sec)
    }
}

impl Timelike for NaiveTime {
    /// Returns the hour number from 0 to 23.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms(0, 0, 0).hour(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano(23, 56, 4, 12_345_678).hour(), 23);
    /// ```
    #[inline]
    fn hour(&self) -> u32 {
        self.hms().0
    }

    /// Returns the minute number from 0 to 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms(0, 0, 0).minute(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano(23, 56, 4, 12_345_678).minute(), 56);
    /// ```
    #[inline]
    fn minute(&self) -> u32 {
        self.hms().1
    }

    /// Returns the second number from 0 to 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms(0, 0, 0).second(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano(23, 56, 4, 12_345_678).second(), 4);
    /// ```
    ///
    /// This method never returns 60 even when it is a leap second.
    /// ([Why?](#leap-second-handling))
    /// Use the proper [formatting method](#method.format) to get a human-readable representation.
    ///
    /// ```
    /// # use chrono::{NaiveTime, Timelike};
    /// let leap = NaiveTime::from_hms_milli(23, 59, 59, 1_000);
    /// assert_eq!(leap.second(), 59);
    /// assert_eq!(leap.format("%H:%M:%S").to_string(), "23:59:60");
    /// ```
    #[inline]
    fn second(&self) -> u32 {
        self.hms().2
    }

    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](#leap-second-handling).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms(0, 0, 0).nanosecond(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano(23, 56, 4, 12_345_678).nanosecond(), 12_345_678);
    /// ```
    ///
    /// Leap seconds may have seemingly out-of-range return values.
    /// You can reduce the range with `time.nanosecond() % 1_000_000_000`, or
    /// use the proper [formatting method](#method.format) to get a human-readable representation.
    ///
    /// ```
    /// # use chrono::{NaiveTime, Timelike};
    /// let leap = NaiveTime::from_hms_milli(23, 59, 59, 1_000);
    /// assert_eq!(leap.nanosecond(), 1_000_000_000);
    /// assert_eq!(leap.format("%H:%M:%S%.9f").to_string(), "23:59:60.000000000");
    /// ```
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.frac
    }

    /// Makes a new `NaiveTime` with the hour number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(dt.with_hour(7), Some(NaiveTime::from_hms_nano(7, 56, 4, 12_345_678)));
    /// assert_eq!(dt.with_hour(24), None);
    /// ```
    #[inline]
    fn with_hour(&self, hour: u32) -> Option<NaiveTime> {
        if hour >= 24 {
            return None;
        }
        let secs = hour * 3600 + self.secs % 3600;
        Some(NaiveTime { secs, ..*self })
    }

    /// Makes a new `NaiveTime` with the minute number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(dt.with_minute(45), Some(NaiveTime::from_hms_nano(23, 45, 4, 12_345_678)));
    /// assert_eq!(dt.with_minute(60), None);
    /// ```
    #[inline]
    fn with_minute(&self, min: u32) -> Option<NaiveTime> {
        if min >= 60 {
            return None;
        }
        let secs = self.secs / 3600 * 3600 + min * 60 + self.secs % 60;
        Some(NaiveTime { secs, ..*self })
    }

    /// Makes a new `NaiveTime` with the second number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    /// As with the [`second`](#method.second) method,
    /// the input range is restricted to 0 through 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(dt.with_second(17), Some(NaiveTime::from_hms_nano(23, 56, 17, 12_345_678)));
    /// assert_eq!(dt.with_second(60), None);
    /// ```
    #[inline]
    fn with_second(&self, sec: u32) -> Option<NaiveTime> {
        if sec >= 60 {
            return None;
        }
        let secs = self.secs / 60 * 60 + sec;
        Some(NaiveTime { secs, ..*self })
    }

    /// Makes a new `NaiveTime` with nanoseconds since the whole non-leap second changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    /// As with the [`nanosecond`](#method.nanosecond) method,
    /// the input range can exceed 1,000,000,000 for leap seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(dt.with_nanosecond(333_333_333),
    ///            Some(NaiveTime::from_hms_nano(23, 56, 4, 333_333_333)));
    /// assert_eq!(dt.with_nanosecond(2_000_000_000), None);
    /// ```
    ///
    /// Leap seconds can theoretically follow *any* whole second.
    /// The following would be a proper leap second at the time zone offset of UTC-00:03:57
    /// (there are several historical examples comparable to this "non-sense" offset),
    /// and therefore is allowed.
    ///
    /// ```
    /// # use chrono::{NaiveTime, Timelike};
    /// # let dt = NaiveTime::from_hms_nano(23, 56, 4, 12_345_678);
    /// assert_eq!(dt.with_nanosecond(1_333_333_333),
    ///            Some(NaiveTime::from_hms_nano(23, 56, 4, 1_333_333_333)));
    /// ```
    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime> {
        if nano >= 2_000_000_000 {
            return None;
        }
        Some(NaiveTime {
            frac: nano,
            ..*self
        })
    }

    /// Returns the number of non-leap seconds past the last midnight.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms(1, 2, 3).num_seconds_from_midnight(),
    ///            3723);
    /// assert_eq!(NaiveTime::from_hms_nano(23, 56, 4, 12_345_678).num_seconds_from_midnight(),
    ///            86164);
    /// assert_eq!(NaiveTime::from_hms_milli(23, 59, 59, 1_000).num_seconds_from_midnight(),
    ///            86399);
    /// ```
    #[inline]
    fn num_seconds_from_midnight(&self) -> u32 {
        self.secs // do not repeat the calculation!
    }
}

/// The `Debug` output of the naive time `t` is the same as
/// [`t.format("%H:%M:%S%.f")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
///
/// assert_eq!(format!("{:?}", NaiveTime::from_hms(23, 56, 4)),              "23:56:04");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_milli(23, 56, 4, 12)),    "23:56:04.012");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_micro(23, 56, 4, 1234)),  "23:56:04.001234");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_nano(23, 56, 4, 123456)), "23:56:04.000123456");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveTime;
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_milli(6, 59, 59, 1_500)), "06:59:60.500");
/// ```
impl fmt::Debug for NaiveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hour, min, sec) = self.hms();
        let (sec, nano) = if self.frac >= 1_000_000_000 {
            (sec + 1, self.frac - 1_000_000_000)
        } else {
            (sec, self.frac)
        };

        write!(f, "{:02}:{:02}:{:02}", hour, min, sec)?;
        if nano == 0 {
            Ok(())
        } else if nano % 1_000_000 == 0 {
            write!(f, ".{:03}", nano / 1_000_000)
        } else if nano % 1_000 == 0 {
            write!(f, ".{:06}", nano / 1_000)
        } else {
            write!(f, ".{:09}", nano)
        }
    }
}

/// The `Display` output of the naive time `t` is the same as
/// [`t.format("%H:%M:%S%.f")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
///
/// assert_eq!(format!("{}", NaiveTime::from_hms(23, 56, 4)),              "23:56:04");
/// assert_eq!(format!("{}", NaiveTime::from_hms_milli(23, 56, 4, 12)),    "23:56:04.012");
/// assert_eq!(format!("{}", NaiveTime::from_hms_micro(23, 56, 4, 1234)),  "23:56:04.001234");
/// assert_eq!(format!("{}", NaiveTime::from_hms_nano(23, 56, 4, 123456)), "23:56:04.000123456");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveTime;
/// assert_eq!(format!("{}", NaiveTime::from_hms_milli(6, 59, 59, 1_500)), "06:59:60.500");
/// ```
impl fmt::Display for NaiveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// The default value for a NaiveTime is midnight, 00:00:00 exactly.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveTime;
///
/// let default_time = NaiveTime::default();
/// assert_eq!(default_time, NaiveTime::from_hms(0, 0, 0));
/// ```
impl Default for NaiveTime {
    fn default() -> Self {
        NaiveTime::from_hms(0, 0, 0)
    }
}
