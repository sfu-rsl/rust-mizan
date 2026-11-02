// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! ISO 8601 calendar date without timezone.

use core::fmt;
#[cfg(any(feature = "alloc", feature = "std", test))]
use num_integer::div_mod_floor;
use num_traits::ToPrimitive;

#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::naive::{IsoWeek, NaiveDateTime, NaiveTime};
use crate::oldtime::Duration as OldDuration;
use crate::{Datelike, Weekday};

use super::internals::{self, DateImpl, Mdf, Of, YearFlags};
use super::isoweek;

const MAX_YEAR: i32 = internals::MAX_YEAR;
const MIN_YEAR: i32 = internals::MIN_YEAR;

/// ISO 8601 calendar date without timezone.
/// Allows for every [proleptic Gregorian date](#calendar-date)
/// from Jan 1, 262145 BCE to Dec 31, 262143 CE.
/// Also supports the conversion from ISO 8601 ordinal and week date.
///
/// # Calendar Date
///
/// The ISO 8601 **calendar date** follows the proleptic Gregorian calendar.
/// It is like a normal civil calendar but note some slight differences:
///
/// * Dates before the Gregorian calendar's inception in 1582 are defined via the extrapolation.
///   Be careful, as historical dates are often noted in the Julian calendar and others
///   and the transition to Gregorian may differ across countries (as late as early 20C).
///
///   (Some example: Both Shakespeare from Britain and Cervantes from Spain seemingly died
///   on the same calendar date---April 23, 1616---but in the different calendar.
///   Britain used the Julian calendar at that time, so Shakespeare's death is later.)
///
/// * ISO 8601 calendars has the year 0, which is 1 BCE (a year before 1 CE).
///   If you need a typical BCE/BC and CE/AD notation for year numbers,
///   use the [`Datelike::year_ce`](../trait.Datelike.html#method.year_ce) method.
///
/// # Week Date
///
/// The ISO 8601 **week date** is a triple of year number, week number
/// and [day of the week](../enum.Weekday.html) with the following rules:
///
/// * A week consists of Monday through Sunday, and is always numbered within some year.
///   The week number ranges from 1 to 52 or 53 depending on the year.
///
/// * The week 1 of given year is defined as the first week containing January 4 of that year,
///   or equivalently, the first week containing four or more days in that year.
///
/// * The year number in the week date may *not* correspond to the actual Gregorian year.
///   For example, January 3, 2016 (Sunday) was on the last (53rd) week of 2015.
///
/// Chrono's date types default to the ISO 8601 [calendar date](#calendar-date),
/// but [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) and
/// [`Datelike::weekday`](../trait.Datelike.html#tymethod.weekday) methods
/// can be used to get the corresponding week date.
///
/// # Ordinal Date
///
/// The ISO 8601 **ordinal date** is a pair of year number and day of the year ("ordinal").
/// The ordinal number ranges from 1 to 365 or 366 depending on the year.
/// The year number is the same as that of the [calendar date](#calendar-date).
///
/// This is currently the internal format of Chrono's date types.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct NaiveDate {
    ymdf: DateImpl, // (year << 13) | of
}

impl NaiveDate {
    /// Makes a new `NaiveDate` from year and packed ordinal-flags, with a verification.
    fn from_of(year: i32, of: Of) -> Option<NaiveDate> {
        if year >= MIN_YEAR && year <= MAX_YEAR && of.valid() {
            let Of(of) = of;
            Some(NaiveDate {
                ymdf: (year << 13) | (of as DateImpl),
            })
        } else {
            None
        }
    }

    /// Makes a new `NaiveDate` from year and packed month-day-flags, with a verification.
    fn from_mdf(year: i32, mdf: Mdf) -> Option<NaiveDate> {
        NaiveDate::from_of(year, mdf.to_of())
    }

    /// Makes a new `NaiveDate` from the [calendar date](#calendar-date)
    /// (year, month and day).
    ///
    /// Panics on the out-of-range date, invalid month and/or day.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// let d = NaiveDate::from_ymd(2015, 3, 14);
    /// assert_eq!(d.year(), 2015);
    /// assert_eq!(d.month(), 3);
    /// assert_eq!(d.day(), 14);
    /// assert_eq!(d.ordinal(), 73); // day of year
    /// assert_eq!(d.iso_week().year(), 2015);
    /// assert_eq!(d.iso_week().week(), 11);
    /// assert_eq!(d.weekday(), Weekday::Sat);
    /// assert_eq!(d.num_days_from_ce(), 735671); // days since January 1, 1 CE
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("invalid or out-of-range date")
    }

    /// Makes a new `NaiveDate` from the [calendar date](#calendar-date)
    /// (year, month and day).
    ///
    /// Returns `None` on the out-of-range date, invalid month and/or day.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let from_ymd_opt = NaiveDate::from_ymd_opt;
    ///
    /// assert!(from_ymd_opt(2015, 3, 14).is_some());
    /// assert!(from_ymd_opt(2015, 0, 14).is_none());
    /// assert!(from_ymd_opt(2015, 2, 29).is_none());
    /// assert!(from_ymd_opt(-4, 2, 29).is_some()); // 5 BCE is a leap year
    /// assert!(from_ymd_opt(400000, 1, 1).is_none());
    /// assert!(from_ymd_opt(-400000, 1, 1).is_none());
    /// ```
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        let flags = YearFlags::from_year(year);
        NaiveDate::from_mdf(year, Mdf::new(month, day, flags))
    }

    /// Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
    /// January 1, 1 being day 1.
    ///
    /// Returns `None` if the date is out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let from_ndays_opt = NaiveDate::from_num_days_from_ce_opt;
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// assert_eq!(from_ndays_opt(730_000),      Some(from_ymd(1999, 9, 3)));
    /// assert_eq!(from_ndays_opt(1),            Some(from_ymd(1, 1, 1)));
    /// assert_eq!(from_ndays_opt(0),            Some(from_ymd(0, 12, 31)));
    /// assert_eq!(from_ndays_opt(-1),           Some(from_ymd(0, 12, 30)));
    /// assert_eq!(from_ndays_opt(100_000_000),  None);
    /// assert_eq!(from_ndays_opt(-100_000_000), None);
    /// ```
    pub fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate> {
        let days = days + 365; // make December 31, 1 BCE equal to day 0
        let (year_div_400, cycle) = div_mod_floor(days, 146_097);
        let (year_mod_400, ordinal) = internals::cycle_to_yo(cycle as u32);
        let flags = YearFlags::from_year_mod_400(year_mod_400 as i32);
        NaiveDate::from_of(
            year_div_400 * 400 + year_mod_400 as i32,
            Of::new(ordinal, flags),
        )
    }

    /// Makes a new `NaiveDateTime` from the current date and given `NaiveTime`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    ///
    /// let d = NaiveDate::from_ymd(2015, 6, 3);
    /// let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
    ///
    /// let dt: NaiveDateTime = d.and_time(t);
    /// assert_eq!(dt.date(), d);
    /// assert_eq!(dt.time(), t);
    /// ```
    #[inline]
    pub fn and_time(&self, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime::new(*self, time)
    }

    /// Makes a new `NaiveDateTime` from the current date, hour, minute and second.
    ///
    /// No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;
    /// use `NaiveDate::and_hms_*` methods with a subsecond parameter instead.
    ///
    /// Panics on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike, Timelike, Weekday};
    ///
    /// let d = NaiveDate::from_ymd(2015, 6, 3);
    ///
    /// let dt: NaiveDateTime = d.and_hms(12, 34, 56);
    /// assert_eq!(dt.year(), 2015);
    /// assert_eq!(dt.weekday(), Weekday::Wed);
    /// assert_eq!(dt.second(), 56);
    /// ```
    #[inline]
    pub fn and_hms(&self, hour: u32, min: u32, sec: u32) -> NaiveDateTime {
        self.and_hms_opt(hour, min, sec).expect("invalid time")
    }

    /// Makes a new `NaiveDateTime` from the current date, hour, minute and second.
    ///
    /// No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;
    /// use `NaiveDate::and_hms_*_opt` methods with a subsecond parameter instead.
    ///
    /// Returns `None` on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd(2015, 6, 3);
    /// assert!(d.and_hms_opt(12, 34, 56).is_some());
    /// assert!(d.and_hms_opt(12, 34, 60).is_none()); // use `and_hms_milli_opt` instead
    /// assert!(d.and_hms_opt(12, 60, 56).is_none());
    /// assert!(d.and_hms_opt(24, 34, 56).is_none());
    /// ```
    #[inline]
    pub fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_opt(hour, min, sec).map(|time| self.and_time(time))
    }

    /// Returns the packed month-day-flags.
    #[inline]
    fn mdf(&self) -> Mdf {
        self.of().to_mdf()
    }

    /// Returns the packed ordinal-flags.
    #[inline]
    fn of(&self) -> Of {
        Of((self.ymdf & 0b1_1111_1111_1111) as u32)
    }

    /// Makes a new `NaiveDate` with the packed month-day-flags changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    #[inline]
    fn with_mdf(&self, mdf: Mdf) -> Option<NaiveDate> {
        self.with_of(mdf.to_of())
    }

    /// Makes a new `NaiveDate` with the packed ordinal-flags changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    #[inline]
    fn with_of(&self, of: Of) -> Option<NaiveDate> {
        if of.valid() {
            let Of(of) = of;
            Some(NaiveDate {
                ymdf: (self.ymdf & !0b1_1111_1111_1111) | of as DateImpl,
            })
        } else {
            None
        }
    }

    /// Adds the `days` part of given `Duration` to the current date.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Duration, NaiveDate};
    ///
    /// let d = NaiveDate::from_ymd(2015, 9, 5);
    /// assert_eq!(d.checked_add_signed(Duration::days(40)),
    ///            Some(NaiveDate::from_ymd(2015, 10, 15)));
    /// assert_eq!(d.checked_add_signed(Duration::days(-40)),
    ///            Some(NaiveDate::from_ymd(2015, 7, 27)));
    /// assert_eq!(d.checked_add_signed(Duration::days(1_000_000_000)), None);
    /// assert_eq!(d.checked_add_signed(Duration::days(-1_000_000_000)), None);
    /// assert_eq!(NaiveDate::MAX.checked_add_signed(Duration::days(1)), None);
    /// ```
    pub fn checked_add_signed(self, rhs: OldDuration) -> Option<NaiveDate> {
        let year = self.year();
        let (mut year_div_400, year_mod_400) = div_mod_floor(year, 400);
        let cycle = internals::yo_to_cycle(year_mod_400 as u32, self.of().ordinal());
        let cycle = try_opt!((cycle as i32).checked_add(try_opt!(rhs.num_days().to_i32())));
        let (cycle_div_400y, cycle) = div_mod_floor(cycle, 146_097);
        year_div_400 += cycle_div_400y;

        let (year_mod_400, ordinal) = internals::cycle_to_yo(cycle as u32);
        let flags = YearFlags::from_year_mod_400(year_mod_400 as i32);
        NaiveDate::from_of(
            year_div_400 * 400 + year_mod_400 as i32,
            Of::new(ordinal, flags),
        )
    }
}

impl Datelike for NaiveDate {
    /// Returns the year number in the [calendar date](#calendar-date).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).year(), 2015);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).year(), -308); // 309 BCE
    /// ```
    #[inline]
    fn year(&self) -> i32 {
        self.ymdf >> 13
    }

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).month(), 9);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).month(), 3);
    /// ```
    #[inline]
    fn month(&self) -> u32 {
        self.mdf().month()
    }

    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).month0(), 8);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).month0(), 2);
    /// ```
    #[inline]
    fn month0(&self) -> u32 {
        self.mdf().month() - 1
    }

    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).day(), 8);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).day(), 14);
    /// ```
    ///
    /// Combined with [`NaiveDate::pred`](#method.pred),
    /// one can determine the number of days in a particular month.
    /// (Note that this panics when `year` is out of range.)
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// fn ndays_in_month(year: i32, month: u32) -> u32 {
    ///     // the first day of the next month...
    ///     let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    ///     let d = NaiveDate::from_ymd(y, m, 1);
    ///
    ///     // ...is preceded by the last day of the original month
    ///     d.pred().day()
    /// }
    ///
    /// assert_eq!(ndays_in_month(2015, 8), 31);
    /// assert_eq!(ndays_in_month(2015, 9), 30);
    /// assert_eq!(ndays_in_month(2015, 12), 31);
    /// assert_eq!(ndays_in_month(2016, 2), 29);
    /// assert_eq!(ndays_in_month(2017, 2), 28);
    /// ```
    #[inline]
    fn day(&self) -> u32 {
        self.mdf().day()
    }

    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).day0(), 7);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).day0(), 13);
    /// ```
    #[inline]
    fn day0(&self) -> u32 {
        self.mdf().day() - 1
    }

    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).ordinal(), 251);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).ordinal(), 74);
    /// ```
    ///
    /// Combined with [`NaiveDate::pred`](#method.pred),
    /// one can determine the number of days in a particular year.
    /// (Note that this panics when `year` is out of range.)
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// fn ndays_in_year(year: i32) -> u32 {
    ///     // the first day of the next year...
    ///     let d = NaiveDate::from_ymd(year + 1, 1, 1);
    ///
    ///     // ...is preceded by the last day of the original year
    ///     d.pred().ordinal()
    /// }
    ///
    /// assert_eq!(ndays_in_year(2015), 365);
    /// assert_eq!(ndays_in_year(2016), 366);
    /// assert_eq!(ndays_in_year(2017), 365);
    /// assert_eq!(ndays_in_year(2000), 366);
    /// assert_eq!(ndays_in_year(2100), 365);
    /// ```
    #[inline]
    fn ordinal(&self) -> u32 {
        self.of().ordinal()
    }

    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).ordinal0(), 250);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).ordinal0(), 73);
    /// ```
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.of().ordinal() - 1
    }

    /// Returns the day of week.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).weekday(), Weekday::Tue);
    /// assert_eq!(NaiveDate::from_ymd(-308, 3, 14).weekday(), Weekday::Fri);
    /// ```
    #[inline]
    fn weekday(&self) -> Weekday {
        self.of().weekday()
    }

    #[inline]
    fn iso_week(&self) -> IsoWeek {
        isoweek::iso_week_from_yof(self.year(), self.of())
    }

    /// Makes a new `NaiveDate` with the year number changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_year(2016),
    ///            Some(NaiveDate::from_ymd(2016, 9, 8)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_year(-308),
    ///            Some(NaiveDate::from_ymd(-308, 9, 8)));
    /// ```
    ///
    /// A leap day (February 29) is a good example that this method can return `None`.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Datelike};
    /// assert!(NaiveDate::from_ymd(2016, 2, 29).with_year(2015).is_none());
    /// assert!(NaiveDate::from_ymd(2016, 2, 29).with_year(2020).is_some());
    /// ```
    #[inline]
    fn with_year(&self, year: i32) -> Option<NaiveDate> {
        // we need to operate with `mdf` since we should keep the month and day number as is
        let mdf = self.mdf();

        // adjust the flags as needed
        let flags = YearFlags::from_year(year);
        let mdf = mdf.with_flags(flags);

        NaiveDate::from_mdf(year, mdf)
    }

    /// Makes a new `NaiveDate` with the month number (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_month(10),
    ///            Some(NaiveDate::from_ymd(2015, 10, 8)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_month(13), None); // no month 13
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 30).with_month(2), None); // no February 30
    /// ```
    #[inline]
    fn with_month(&self, month: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_month(month))
    }

    /// Makes a new `NaiveDate` with the month number (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_month0(9),
    ///            Some(NaiveDate::from_ymd(2015, 10, 8)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_month0(12), None); // no month 13
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 30).with_month0(1), None); // no February 30
    /// ```
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_month(month0 + 1))
    }

    /// Makes a new `NaiveDate` with the day of month (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_day(30),
    ///            Some(NaiveDate::from_ymd(2015, 9, 30)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_day(31),
    ///            None); // no September 31
    /// ```
    #[inline]
    fn with_day(&self, day: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_day(day))
    }

    /// Makes a new `NaiveDate` with the day of month (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_day0(29),
    ///            Some(NaiveDate::from_ymd(2015, 9, 30)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 9, 8).with_day0(30),
    ///            None); // no September 31
    /// ```
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_day(day0 + 1))
    }

    /// Makes a new `NaiveDate` with the day of year (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 1, 1).with_ordinal(60),
    ///            Some(NaiveDate::from_ymd(2015, 3, 1)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 1, 1).with_ordinal(366),
    ///            None); // 2015 had only 365 days
    ///
    /// assert_eq!(NaiveDate::from_ymd(2016, 1, 1).with_ordinal(60),
    ///            Some(NaiveDate::from_ymd(2016, 2, 29)));
    /// assert_eq!(NaiveDate::from_ymd(2016, 1, 1).with_ordinal(366),
    ///            Some(NaiveDate::from_ymd(2016, 12, 31)));
    /// ```
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate> {
        self.with_of(self.of().with_ordinal(ordinal))
    }

    /// Makes a new `NaiveDate` with the day of year (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(2015, 1, 1).with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd(2015, 3, 1)));
    /// assert_eq!(NaiveDate::from_ymd(2015, 1, 1).with_ordinal0(365),
    ///            None); // 2015 had only 365 days
    ///
    /// assert_eq!(NaiveDate::from_ymd(2016, 1, 1).with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd(2016, 2, 29)));
    /// assert_eq!(NaiveDate::from_ymd(2016, 1, 1).with_ordinal0(365),
    ///            Some(NaiveDate::from_ymd(2016, 12, 31)));
    /// ```
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate> {
        self.with_of(self.of().with_ordinal(ordinal0 + 1))
    }
}

// TODO: NaiveDateDaysIterator and NaiveDateWeeksIterator should implement FusedIterator,
// TrustedLen, and Step once they becomes stable.
// See: https://github.com/chronotope/chrono/issues/208

/// The `Debug` output of the naive date `d` is the same as
/// [`d.format("%Y-%m-%d")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd(2015,  9,  5)), "2015-09-05");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd(   0,  1,  1)), "0000-01-01");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd(9999, 12, 31)), "9999-12-31");
/// ```
///
/// ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE.
///
/// ```
/// # use chrono::NaiveDate;
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd(   -1,  1,  1)),  "-0001-01-01");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd(10000, 12, 31)), "+10000-12-31");
/// ```
impl fmt::Debug for NaiveDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let year = self.year();
        let mdf = self.mdf();
        if 0 <= year && year <= 9999 {
            write!(f, "{:04}-{:02}-{:02}", year, mdf.month(), mdf.day())
        } else {
            // ISO 8601 requires the explicit sign for out-of-range years
            write!(f, "{:+05}-{:02}-{:02}", year, mdf.month(), mdf.day())
        }
    }
}

/// The `Display` output of the naive date `d` is the same as
/// [`d.format("%Y-%m-%d")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// assert_eq!(format!("{}", NaiveDate::from_ymd(2015,  9,  5)), "2015-09-05");
/// assert_eq!(format!("{}", NaiveDate::from_ymd(   0,  1,  1)), "0000-01-01");
/// assert_eq!(format!("{}", NaiveDate::from_ymd(9999, 12, 31)), "9999-12-31");
/// ```
///
/// ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE.
///
/// ```
/// # use chrono::NaiveDate;
/// assert_eq!(format!("{}", NaiveDate::from_ymd(   -1,  1,  1)),  "-0001-01-01");
/// assert_eq!(format!("{}", NaiveDate::from_ymd(10000, 12, 31)), "+10000-12-31");
/// ```
impl fmt::Display for NaiveDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// The default value for a NaiveDate is 1st of January 1970.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDate;
///
/// let default_date = NaiveDate::default();
/// assert_eq!(default_date, NaiveDate::from_ymd(1970, 1, 1));
/// ```
impl Default for NaiveDate {
    fn default() -> Self {
        NaiveDate::from_ymd(1970, 1, 1)
    }
}
