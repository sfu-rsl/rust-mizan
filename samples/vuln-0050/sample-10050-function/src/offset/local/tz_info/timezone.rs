//! Types related to a time zone.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::{cmp::Ordering, fmt, str};

use super::rule::{AlternateTime, TransitionRule};
use super::{parser, Error, DAYS_PER_WEEK, SECONDS_PER_DAY};

/// Time zone
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct TimeZone {
    /// List of transitions
    transitions: Vec<Transition>,
    /// List of local time types (cannot be empty)
    local_time_types: Vec<LocalTimeType>,
    /// List of leap seconds
    leap_seconds: Vec<LeapSecond>,
    /// Extra transition rule applicable after the last transition
    extra_rule: Option<TransitionRule>,
}

impl TimeZone {
    /// Returns local time zone.
    ///
    /// This method in not supported on non-UNIX platforms, and returns the UTC time zone instead.
    ///
    pub(crate) fn local() -> Result<Self, Error> {
        if let Ok(tz) = std::env::var("TZ") {
            Self::from_posix_tz(&tz)
        } else {
            Self::from_posix_tz("localtime")
        }
    }

    /// Construct a time zone from a POSIX TZ string, as described in [the POSIX documentation of the `TZ` environment variable](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap08.html).
    fn from_posix_tz(tz_string: &str) -> Result<Self, Error> {
        if tz_string.is_empty() {
            return Err(Error::InvalidTzString("empty TZ string"));
        }

        if tz_string == "localtime" {
            return Self::from_tz_data(&fs::read("/etc/localtime")?);
        }

        let mut chars = tz_string.chars();
        if chars.next() == Some(':') {
            return Self::from_file(&mut find_tz_file(chars.as_str())?);
        }

        if let Ok(mut file) = find_tz_file(tz_string) {
            return Self::from_file(&mut file);
        }

        // TZ string extensions are not allowed
        let tz_string = tz_string.trim_matches(|c: char| c.is_ascii_whitespace());
        let rule = TransitionRule::from_tz_string(tz_string.as_bytes(), false)?;
        Self::new(
            vec![],
            match rule {
                TransitionRule::Fixed(local_time_type) => vec![local_time_type],
                TransitionRule::Alternate(AlternateTime { std, dst, .. }) => vec![std, dst],
            },
            vec![],
            Some(rule),
        )
    }

    /// Construct a time zone
    pub(super) fn new(
        transitions: Vec<Transition>,
        local_time_types: Vec<LocalTimeType>,
        leap_seconds: Vec<LeapSecond>,
        extra_rule: Option<TransitionRule>,
    ) -> Result<Self, Error> {
        let new = Self {
            transitions,
            local_time_types,
            leap_seconds,
            extra_rule,
        };
        new.as_ref().validate()?;
        Ok(new)
    }

    /// Construct a time zone from the contents of a time zone file
    fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Self::from_tz_data(&bytes)
    }

    /// Construct a time zone from the contents of a time zone file
    ///
    /// Parse TZif data as described in [RFC 8536](https://datatracker.ietf.org/doc/html/rfc8536).
    pub(super) fn from_tz_data(bytes: &[u8]) -> Result<Self, Error> {
        parser::parse(bytes)
    }

    /// Construct a time zone with the specified UTC offset in seconds
    fn fixed(ut_offset: i32) -> Result<Self, Error> {
        Ok(Self {
            transitions: Vec::new(),
            local_time_types: vec![LocalTimeType::with_offset(ut_offset)?],
            leap_seconds: Vec::new(),
            extra_rule: None,
        })
    }

    /// Construct the time zone associated to UTC
    fn utc() -> Self {
        Self {
            transitions: Vec::new(),
            local_time_types: vec![LocalTimeType::UTC],
            leap_seconds: Vec::new(),
            extra_rule: None,
        }
    }

    /// Find the local time type associated to the time zone at the specified Unix time in seconds
    pub(crate) fn find_local_time_type(&self, unix_time: i64) -> Result<&LocalTimeType, Error> {
        self.as_ref().find_local_time_type(unix_time)
    }

    // should we pass NaiveDateTime all the way through to this fn?
    pub(crate) fn find_local_time_type_from_local(
        &self,
        local_time: i64,
        year: i32,
    ) -> Result<crate::LocalResult<LocalTimeType>, Error> {
        self.as_ref()
            .find_local_time_type_from_local(local_time, year)
    }

    /// Returns a reference to the time zone
    fn as_ref(&self) -> TimeZoneRef {
        TimeZoneRef {
            transitions: &self.transitions,
            local_time_types: &self.local_time_types,
            leap_seconds: &self.leap_seconds,
            extra_rule: &self.extra_rule,
        }
    }
}

/// Reference to a time zone
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct TimeZoneRef<'a> {
    /// List of transitions
    transitions: &'a [Transition],
    /// List of local time types (cannot be empty)
    local_time_types: &'a [LocalTimeType],
    /// List of leap seconds
    leap_seconds: &'a [LeapSecond],
    /// Extra transition rule applicable after the last transition
    extra_rule: &'a Option<TransitionRule>,
}

impl<'a> TimeZoneRef<'a> {
    /// Find the local time type associated to the time zone at the specified Unix time in seconds
    pub(crate) fn find_local_time_type(&self, unix_time: i64) -> Result<&'a LocalTimeType, Error> {
        let extra_rule = match self.transitions.last() {
            None => match self.extra_rule {
                Some(extra_rule) => extra_rule,
                None => return Ok(&self.local_time_types[0]),
            },
            Some(last_transition) => {
                let unix_leap_time = match self.unix_time_to_unix_leap_time(unix_time) {
                    Ok(unix_leap_time) => unix_leap_time,
                    Err(Error::OutOfRange(error)) => return Err(Error::FindLocalTimeType(error)),
                    Err(err) => return Err(err),
                };

                if unix_leap_time >= last_transition.unix_leap_time {
                    match self.extra_rule {
                        Some(extra_rule) => extra_rule,
                        None => {
                            return Err(Error::FindLocalTimeType(
                                "no local time type is available for the specified timestamp",
                            ))
                        }
                    }
                } else {
                    let index = match self
                        .transitions
                        .binary_search_by_key(&unix_leap_time, Transition::unix_leap_time)
                    {
                        Ok(x) => x + 1,
                        Err(x) => x,
                    };

                    let local_time_type_index = if index > 0 {
                        self.transitions[index - 1].local_time_type_index
                    } else {
                        0
                    };
                    return Ok(&self.local_time_types[local_time_type_index]);
                }
            }
        };

        match extra_rule.find_local_time_type(unix_time) {
            Ok(local_time_type) => Ok(local_time_type),
            Err(Error::OutOfRange(error)) => Err(Error::FindLocalTimeType(error)),
            err => err,
        }
    }

    pub(crate) fn find_local_time_type_from_local(
        &self,
        local_time: i64,
        year: i32,
    ) -> Result<crate::LocalResult<LocalTimeType>, Error> {
        // #TODO: this is wrong as we need 'local_time_to_local_leap_time ?
        // but ... does the local time even include leap seconds ??
        // let unix_leap_time = match self.unix_time_to_unix_leap_time(local_time) {
        //     Ok(unix_leap_time) => unix_leap_time,
        //     Err(Error::OutOfRange(error)) => return Err(Error::FindLocalTimeType(error)),
        //     Err(err) => return Err(err),
        // };
        let local_leap_time = local_time;

        // if we have at least one transition,
        // we must check _all_ of them, incase of any Overlapping (LocalResult::Ambiguous) or Skipping (LocalResult::None) transitions
        if !self.transitions.is_empty() {
            let mut prev = Some(self.local_time_types[0]);

            for transition in self.transitions {
                let after_ltt = self.local_time_types[transition.local_time_type_index];

                // the end and start here refers to where the time starts prior to the transition
                // and where it ends up after. not the temporal relationship.
                let transition_end = transition.unix_leap_time + i64::from(after_ltt.ut_offset);
                let transition_start =
                    transition.unix_leap_time + i64::from(prev.unwrap().ut_offset);

                match transition_start.cmp(&transition_end) {
                    Ordering::Greater => {
                        // bakwards transition, eg from DST to regular
                        // this means a given local time could have one of two possible offsets
                        if local_leap_time < transition_end {
                            return Ok(crate::LocalResult::Single(prev.unwrap()));
                        } else if local_leap_time >= transition_end
                            && local_leap_time <= transition_start
                        {
                            if prev.unwrap().ut_offset < after_ltt.ut_offset {
                                return Ok(crate::LocalResult::Ambiguous(prev.unwrap(), after_ltt));
                            } else {
                                return Ok(crate::LocalResult::Ambiguous(after_ltt, prev.unwrap()));
                            }
                        }
                    }
                    Ordering::Equal => {
                        // should this ever happen? presumably we have to handle it anyway.
                        if local_leap_time < transition_start {
                            return Ok(crate::LocalResult::Single(prev.unwrap()));
                        } else if local_leap_time == transition_end {
                            if prev.unwrap().ut_offset < after_ltt.ut_offset {
                                return Ok(crate::LocalResult::Ambiguous(prev.unwrap(), after_ltt));
                            } else {
                                return Ok(crate::LocalResult::Ambiguous(after_ltt, prev.unwrap()));
                            }
                        }
                    }
                    Ordering::Less => {
                        // forwards transition, eg from regular to DST
                        // this means that times that are skipped are invalid local times
                        if local_leap_time <= transition_start {
                            return Ok(crate::LocalResult::Single(prev.unwrap()));
                        } else if local_leap_time < transition_end {
                            return Ok(crate::LocalResult::None);
                        } else if local_leap_time == transition_end {
                            return Ok(crate::LocalResult::Single(after_ltt));
                        }
                    }
                }

                // try the next transition, we are fully after this one
                prev = Some(after_ltt);
            }
        };

        if let Some(extra_rule) = self.extra_rule {
            match extra_rule.find_local_time_type_from_local(local_time, year) {
                Ok(local_time_type) => Ok(local_time_type),
                Err(Error::OutOfRange(error)) => Err(Error::FindLocalTimeType(error)),
                err => err,
            }
        } else {
            Ok(crate::LocalResult::Single(self.local_time_types[0]))
        }
    }

    /// Check time zone inputs
    fn validate(&self) -> Result<(), Error> {
        // Check local time types
        let local_time_types_size = self.local_time_types.len();
        if local_time_types_size == 0 {
            return Err(Error::TimeZone(
                "list of local time types must not be empty",
            ));
        }

        // Check transitions
        let mut i_transition = 0;
        while i_transition < self.transitions.len() {
            if self.transitions[i_transition].local_time_type_index >= local_time_types_size {
                return Err(Error::TimeZone("invalid local time type index"));
            }

            if i_transition + 1 < self.transitions.len()
                && self.transitions[i_transition].unix_leap_time
                    >= self.transitions[i_transition + 1].unix_leap_time
            {
                return Err(Error::TimeZone("invalid transition"));
            }

            i_transition += 1;
        }

        // Check leap seconds
        if !(self.leap_seconds.is_empty()
            || self.leap_seconds[0].unix_leap_time >= 0
                && saturating_abs(self.leap_seconds[0].correction) == 1)
        {
            return Err(Error::TimeZone("invalid leap second"));
        }

        let min_interval = SECONDS_PER_28_DAYS - 1;

        let mut i_leap_second = 0;
        while i_leap_second < self.leap_seconds.len() {
            if i_leap_second + 1 < self.leap_seconds.len() {
                let x0 = &self.leap_seconds[i_leap_second];
                let x1 = &self.leap_seconds[i_leap_second + 1];

                let diff_unix_leap_time = x1.unix_leap_time.saturating_sub(x0.unix_leap_time);
                let abs_diff_correction =
                    saturating_abs(x1.correction.saturating_sub(x0.correction));

                if !(diff_unix_leap_time >= min_interval && abs_diff_correction == 1) {
                    return Err(Error::TimeZone("invalid leap second"));
                }
            }
            i_leap_second += 1;
        }

        // Check extra rule
        let (extra_rule, last_transition) = match (&self.extra_rule, self.transitions.last()) {
            (Some(rule), Some(trans)) => (rule, trans),
            _ => return Ok(()),
        };

        let last_local_time_type = &self.local_time_types[last_transition.local_time_type_index];
        let unix_time = match self.unix_leap_time_to_unix_time(last_transition.unix_leap_time) {
            Ok(unix_time) => unix_time,
            Err(Error::OutOfRange(error)) => return Err(Error::TimeZone(error)),
            Err(err) => return Err(err),
        };

        let rule_local_time_type = match extra_rule.find_local_time_type(unix_time) {
            Ok(rule_local_time_type) => rule_local_time_type,
            Err(Error::OutOfRange(error)) => return Err(Error::TimeZone(error)),
            Err(err) => return Err(err),
        };

        let check = last_local_time_type.ut_offset == rule_local_time_type.ut_offset
            && last_local_time_type.is_dst == rule_local_time_type.is_dst
            && match (&last_local_time_type.name, &rule_local_time_type.name) {
                (Some(x), Some(y)) => x.equal(y),
                (None, None) => true,
                _ => false,
            };

        if !check {
            return Err(Error::TimeZone(
                "extra transition rule is inconsistent with the last transition",
            ));
        }

        Ok(())
    }

    /// Convert Unix time to Unix leap time, from the list of leap seconds in a time zone
    fn unix_time_to_unix_leap_time(&self, unix_time: i64) -> Result<i64, Error> {
        let mut unix_leap_time = unix_time;

        let mut i = 0;
        while i < self.leap_seconds.len() {
            let leap_second = &self.leap_seconds[i];

            if unix_leap_time < leap_second.unix_leap_time {
                break;
            }

            unix_leap_time = match unix_time.checked_add(leap_second.correction as i64) {
                Some(unix_leap_time) => unix_leap_time,
                None => return Err(Error::OutOfRange("out of range operation")),
            };

            i += 1;
        }

        Ok(unix_leap_time)
    }

    /// Convert Unix leap time to Unix time, from the list of leap seconds in a time zone
    fn unix_leap_time_to_unix_time(&self, unix_leap_time: i64) -> Result<i64, Error> {
        if unix_leap_time == i64::min_value() {
            return Err(Error::OutOfRange("out of range operation"));
        }

        let index = match self
            .leap_seconds
            .binary_search_by_key(&(unix_leap_time - 1), LeapSecond::unix_leap_time)
        {
            Ok(x) => x + 1,
            Err(x) => x,
        };

        let correction = if index > 0 {
            self.leap_seconds[index - 1].correction
        } else {
            0
        };

        match unix_leap_time.checked_sub(correction as i64) {
            Some(unix_time) => Ok(unix_time),
            None => Err(Error::OutOfRange("out of range operation")),
        }
    }

    /// The UTC time zone
    const UTC: TimeZoneRef<'static> = TimeZoneRef {
        transitions: &[],
        local_time_types: &[LocalTimeType::UTC],
        leap_seconds: &[],
        extra_rule: &None,
    };
}

/// Transition of a TZif file
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct Transition {
    /// Unix leap time
    unix_leap_time: i64,
    /// Index specifying the local time type of the transition
    local_time_type_index: usize,
}

impl Transition {
    /// Construct a TZif file transition
    pub(super) fn new(unix_leap_time: i64, local_time_type_index: usize) -> Self {
        Self {
            unix_leap_time,
            local_time_type_index,
        }
    }

    /// Returns Unix leap time
    fn unix_leap_time(&self) -> i64 {
        self.unix_leap_time
    }
}

/// Leap second of a TZif file
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct LeapSecond {
    /// Unix leap time
    unix_leap_time: i64,
    /// Leap second correction
    correction: i32,
}

impl LeapSecond {
    /// Construct a TZif file leap second
    pub(super) fn new(unix_leap_time: i64, correction: i32) -> Self {
        Self {
            unix_leap_time,
            correction,
        }
    }

    /// Returns Unix leap time
    fn unix_leap_time(&self) -> i64 {
        self.unix_leap_time
    }
}

/// ASCII-encoded fixed-capacity string, used for storing time zone names
#[derive(Copy, Clone, Eq, PartialEq)]
struct TimeZoneName {
    /// Length-prefixed string buffer
    bytes: [u8; 8],
}

impl TimeZoneName {
    /// Construct a time zone name
    fn new(input: &[u8]) -> Result<Self, Error> {
        let len = input.len();

        if len < 3 || len > 7 {
            return Err(Error::LocalTimeType(
                "time zone name must have between 3 and 7 characters",
            ));
        }

        let mut bytes = [0; 8];
        bytes[0] = input.len() as u8;

        let mut i = 0;
        while i < len {
            let b = input[i];
            match b {
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'+' | b'-' => {}
                _ => return Err(Error::LocalTimeType("invalid characters in time zone name")),
            }

            bytes[i + 1] = b;
            i += 1;
        }

        Ok(Self { bytes })
    }

    /// Returns time zone name as a byte slice
    fn as_bytes(&self) -> &[u8] {
        match self.bytes[0] {
            3 => &self.bytes[1..4],
            4 => &self.bytes[1..5],
            5 => &self.bytes[1..6],
            6 => &self.bytes[1..7],
            7 => &self.bytes[1..8],
            _ => unreachable!(),
        }
    }

    /// Check if two time zone names are equal
    fn equal(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl AsRef<str> for TimeZoneName {
    fn as_ref(&self) -> &str {
        // SAFETY: ASCII is valid UTF-8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl fmt::Debug for TimeZoneName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

/// Local time type associated to a time zone
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct LocalTimeType {
    /// Offset from UTC in seconds
    pub(super) ut_offset: i32,
    /// Daylight Saving Time indicator
    is_dst: bool,
    /// Time zone name
    name: Option<TimeZoneName>,
}

impl LocalTimeType {
    /// Construct a local time type
    pub(super) fn new(ut_offset: i32, is_dst: bool, name: Option<&[u8]>) -> Result<Self, Error> {
        if ut_offset == i32::min_value() {
            return Err(Error::LocalTimeType("invalid UTC offset"));
        }

        let name = match name {
            Some(name) => TimeZoneName::new(name)?,
            None => {
                return Ok(Self {
                    ut_offset,
                    is_dst,
                    name: None,
                })
            }
        };

        Ok(Self {
            ut_offset,
            is_dst,
            name: Some(name),
        })
    }

    /// Construct a local time type with the specified UTC offset in seconds
    pub(super) fn with_offset(ut_offset: i32) -> Result<Self, Error> {
        if ut_offset == i32::min_value() {
            return Err(Error::LocalTimeType("invalid UTC offset"));
        }

        Ok(Self {
            ut_offset,
            is_dst: false,
            name: None,
        })
    }

    /// Returns offset from UTC in seconds
    pub(crate) fn offset(&self) -> i32 {
        self.ut_offset
    }

    /// Returns daylight saving time indicator
    pub(super) fn is_dst(&self) -> bool {
        self.is_dst
    }

    pub(super) const UTC: LocalTimeType = Self {
        ut_offset: 0,
        is_dst: false,
        name: None,
    };
}

/// Open the TZif file corresponding to a TZ string
fn find_tz_file(path: impl AsRef<Path>) -> Result<File, Error> {
    // Don't check system timezone directories on non-UNIX platforms
    #[cfg(not(unix))]
    return Ok(File::open(path)?);

    #[cfg(unix)]
    {
        let path = path.as_ref();
        if path.is_absolute() {
            return Ok(File::open(path)?);
        }

        for folder in &ZONE_INFO_DIRECTORIES {
            if let Ok(file) = File::open(PathBuf::from(folder).join(path)) {
                return Ok(file);
            }
        }

        Err(Error::Io(io::ErrorKind::NotFound.into()))
    }
}

#[inline]
fn saturating_abs(v: i32) -> i32 {
    if v.is_positive() {
        v
    } else if v == i32::min_value() {
        i32::max_value()
    } else {
        -v
    }
}

// Possible system timezone directories
#[cfg(unix)]
const ZONE_INFO_DIRECTORIES: [&str; 3] =
    ["/usr/share/zoneinfo", "/share/zoneinfo", "/etc/zoneinfo"];

/// Number of seconds in one week
pub(crate) const SECONDS_PER_WEEK: i64 = SECONDS_PER_DAY * DAYS_PER_WEEK;
/// Number of seconds in 28 days
const SECONDS_PER_28_DAYS: i64 = SECONDS_PER_DAY * 28;
