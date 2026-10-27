//! Points in time, calendar dates, and the dates and relative times the
//! browser writes for them with `Intl`.

use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;

use crate::expr::{Expr, IntoExpr};
use crate::intl::IntlLocale;
use crate::signal::Sig;

/// A point in time, in milliseconds since the Unix epoch.
///
/// It counts time as JavaScript's `Date` does, and serializes as a JSON number, which the browser holds exactly for any
/// time `Date` can hold.
///
/// ```
/// use hypersig::Timestamp;
///
/// let new_year = Timestamp::from_millis(1_767_225_600_000);
/// assert_eq!(new_year.as_millis(), 1_767_225_600_000);
/// assert!(Timestamp::now() > Timestamp::from_millis(0));
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Timestamp(i64);

impl Timestamp {
    /// The time `millis` milliseconds after the Unix epoch.
    ///
    /// A negative `millis` counts back before the epoch.
    #[must_use]
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// The milliseconds since the Unix epoch.
    #[must_use]
    pub const fn as_millis(self) -> i64 {
        self.0
    }

    /// The server's current time.
    #[must_use]
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }

    /// The server's current time.
    #[must_use]
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn now() -> Self {
        // `SystemTime::now` panics on this target, and `Date.now()` returns
        // whole milliseconds as a float.
        #[expect(
            clippy::cast_possible_truncation,
            reason = "Date.now() is a whole number of milliseconds within i64"
        )]
        Self(js_sys::Date::now() as i64)
    }
}

impl From<SystemTime> for Timestamp {
    /// Rounds `time` down to whole milliseconds, as `Math.floor` would, and
    /// saturates past `i64`'s range of milliseconds.
    fn from(time: SystemTime) -> Self {
        let millis = |duration: Duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX);
        match time.duration_since(UNIX_EPOCH) {
            Ok(after) => Self(millis(after)),
            Err(before) => {
                let before = before.duration();
                let partial = before.subsec_nanos() % 1_000_000 != 0;
                Self((-millis(before)).saturating_sub(i64::from(partial)))
            }
        }
    }
}

impl<S> IntoExpr<S, Timestamp> for Timestamp {
    fn into_expr(self) -> Expr<S, Timestamp> {
        Expr::build(|_| json!(["lit", self.0]))
    }
}

impl<S> Expr<S, Timestamp> {
    /// The browser's current time, which the server leaves out of the first HTML.
    ///
    /// The server cannot know when the browser evaluates the expression. A
    /// page whose first HTML shows a time relative to now keeps the server's
    /// [`Timestamp::now`] in a signal instead.
    #[must_use]
    pub fn timestamp_now() -> Self {
        Self::build(|_| json!(["now"]))
    }
}

/// A day of the calendar, with no time of day and no time zone.
///
/// It is the value of an `<input type="date">`, which
/// [`El::bind`](crate::html::El::bind) binds to it, and serializes as the
/// text that field holds, such as `"2026-01-05"`. The year runs from 1 to
/// 9999, which the field writes in four digits, so two dates compare in
/// order as text too.
///
/// ```
/// use hypersig::CalendarDate;
///
/// const NEW_YEAR: CalendarDate = CalendarDate::new(2026, 1, 1);
/// assert_eq!(NEW_YEAR.to_string(), "2026-01-01");
/// assert_eq!(CalendarDate::parse("2026-01-05").map(|date| date.day()), Ok(5));
/// assert!(CalendarDate::parse("2026-02-29").is_err());
/// assert!(NEW_YEAR < CalendarDate::new(2026, 1, 5));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalendarDate {
    year: u16,
    month: u8,
    day: u8,
}

impl CalendarDate {
    /// The day `day` of month `month` of year `year`, written in the program.
    ///
    /// # Panics
    ///
    /// Panics if the year is not 1 to 9999, the month not 1 to 12, or the
    /// month has no such day, as 30 February. In a `const` the panic is a
    /// compile error.
    #[must_use]
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        if let Err(reason) = check_date(year, month, day) {
            panic!("{}", reason);
        }
        Self { year, month, day }
    }

    /// Reads a date written as `YYYY-MM-DD`, such as the value of a date field.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarDateError`] where `text` is not shaped as
    /// `YYYY-MM-DD`, or where [`CalendarDate::new`] would panic on its numbers.
    pub fn parse(text: &str) -> Result<Self, CalendarDateError> {
        let error = |reason| CalendarDateError {
            text: text.to_owned(),
            reason,
        };
        let bytes = text.as_bytes();
        let shaped = bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes
                .iter()
                .enumerate()
                .all(|(at, byte)| at == 4 || at == 7 || byte.is_ascii_digit());
        if !shaped {
            return Err(error("it is not shaped as `YYYY-MM-DD`"));
        }
        let number = |range: std::ops::Range<usize>| {
            bytes[range]
                .iter()
                .fold(0, |sum, digit| sum * 10 + u16::from(digit - b'0'))
        };
        let year = number(0..4);
        // Two digits hold at most 99.
        let [_, month] = number(5..7).to_be_bytes();
        let [_, day] = number(8..10).to_be_bytes();
        check_date(year, month, day).map_err(error)?;
        Ok(Self { year, month, day })
    }

    /// The year, from 1 to 9999.
    #[must_use]
    pub const fn year(self) -> u16 {
        self.year
    }

    /// The month, from 1 for January to 12 for December.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, from 1.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }
}

impl fmt::Display for CalendarDate {
    /// Writes the date as `YYYY-MM-DD`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Serialize for CalendarDate {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for CalendarDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Text;
        impl Visitor<'_> for Text {
            type Value = CalendarDate;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a date written as `YYYY-MM-DD`")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<CalendarDate, E> {
                CalendarDate::parse(v).map_err(E::custom)
            }
        }
        deserializer.deserialize_str(Text)
    }
}

impl<S> IntoExpr<S, CalendarDate> for CalendarDate {
    fn into_expr(self) -> Expr<S, CalendarDate> {
        Expr::build(|_| json!(["lit", self.to_string()]))
    }
}

/// Text that [`CalendarDate::parse`] refused, with the rule it broke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDateError {
    text: String,
    reason: &'static str,
}

impl fmt::Display for CalendarDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a calendar date: {}", self.text, self.reason)
    }
}

impl std::error::Error for CalendarDateError {}

/// Whether `year`, `month`, and `day` name a day [`CalendarDate`] holds.
const fn check_date(year: u16, month: u8, day: u8) -> Result<(), &'static str> {
    if year < 1 || year > 9999 {
        return Err("the year is not 1 to 9999");
    }
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return Err("the month is not 1 to 12"),
    };
    if day < 1 || day > days {
        return Err("the month has no such day");
    }
    Ok(())
}

/// How much of the date [`DateFormat`] writes, as `Intl.DateTimeFormat`'s `dateStyle`.
///
/// For 5 January 2026:
///
/// | Style | `IntlLocale::En` | `IntlLocale::Ja` |
/// |---|---|---|
/// | `Full` | Monday, January 5, 2026 | 2026年1月5日月曜日 |
/// | `Long` | January 5, 2026 | 2026年1月5日 |
/// | `Medium` | Jan 5, 2026 | 2026/01/05 |
/// | `Short` | 1/5/26 | 2026/01/05 |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateStyle {
    /// The weekday, the month's name, the day, and the year.
    Full,
    /// The month's name, the day, and the year.
    Long,
    /// The month's short name, the day, and the year.
    Medium,
    /// The month, the day, and the year in digits.
    Short,
}

impl DateStyle {
    const fn name(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Long => "long",
            Self::Medium => "medium",
            Self::Short => "short",
        }
    }
}

/// How much of the time of day [`DateFormat`] writes, as `Intl.DateTimeFormat`'s `timeStyle`.
///
/// `Intl`'s `full` and `long` styles add the zone's name, which a zone
/// given as an offset does not have, so they are left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeStyle {
    /// Hours, minutes, and seconds: `3:04:05 PM` in English, `15:04:05` in Japanese.
    Medium,
    /// Hours and minutes: `3:04 PM` in English, `15:04` in Japanese.
    Short,
}

impl TimeStyle {
    const fn name(self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Short => "short",
        }
    }
}

/// The time zone [`DateFormat`] writes a time in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeZone {
    /// Coordinated Universal Time.
    Utc,
    /// A fixed number of minutes east of UTC, such as `540` for Japan.
    ///
    /// A fixed offset does not follow daylight saving time.
    Offset(i16),
    /// The zone the visitor's browser is set to, which the server does not know.
    ///
    /// The server leaves such a time out of the first HTML, and the runtime
    /// writes it when the page starts.
    Browser,
}

/// How [`format_date`](Expr::format_date) writes a [`Timestamp`]: a locale, styles, and a time zone.
///
/// The browser writes the text with `Intl.DateTimeFormat`, and the server
/// writes the first HTML from rules this crate holds for each
/// [`IntlLocale`]. Browsers carry their own copy of the Unicode CLDR data,
/// and a version that differs in a space or a comma replaces the server's
/// text when the page starts.
///
/// ```
/// use hypersig::{DateFormat, DateStyle, IntlLocale, TimeStyle, TimeZone, Timestamp};
///
/// const POSTED: DateFormat =
///     DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Short);
/// const TOKYO: DateFormat =
///     DateFormat::date(IntlLocale::Ja, DateStyle::Long).zone(TimeZone::Offset(540));
/// // 2025-12-31 15:00 UTC, which is 2026-01-01 00:00 in Tokyo.
/// let time = Timestamp::from_millis(1_767_193_200_000);
/// assert_eq!(POSTED.write(time).as_deref(), Some("Dec 31, 2025, 3:00 PM"));
/// assert_eq!(TOKYO.write(time).as_deref(), Some("2026年1月1日"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateFormat {
    locale: IntlLocale,
    date: Option<DateStyle>,
    time: Option<TimeStyle>,
    zone: TimeZone,
}

impl DateFormat {
    /// The date alone, in UTC until [`zone`](Self::zone) is set.
    #[must_use]
    pub const fn date(locale: IntlLocale, style: DateStyle) -> Self {
        Self {
            locale,
            date: Some(style),
            time: None,
            zone: TimeZone::Utc,
        }
    }

    /// The time of day alone, in UTC until [`zone`](Self::zone) is set.
    #[must_use]
    pub const fn time(locale: IntlLocale, style: TimeStyle) -> Self {
        Self {
            locale,
            date: None,
            time: Some(style),
            zone: TimeZone::Utc,
        }
    }

    /// The date followed by the time of day, in UTC until [`zone`](Self::zone) is set.
    #[must_use]
    pub const fn date_time(locale: IntlLocale, date: DateStyle, time: TimeStyle) -> Self {
        Self {
            locale,
            date: Some(date),
            time: Some(time),
            zone: TimeZone::Utc,
        }
    }

    /// Writes the time in `zone`.
    #[must_use]
    pub const fn zone(mut self, zone: TimeZone) -> Self {
        self.zone = zone;
        self
    }

    /// `time` in this format, or `None` where the server cannot write it.
    ///
    /// The text is what the browser writes. The server cannot write a time
    /// in [`TimeZone::Browser`], or a year before 1000 or after 9999.
    #[must_use]
    pub fn write(&self, time: Timestamp) -> Option<String> {
        let offset = self.offset()?;
        write_date(
            self.locale.tag(),
            self.date.map(DateStyle::name),
            self.time.map(TimeStyle::name),
            time.0.checked_add(i64::from(offset) * MILLIS_PER_MINUTE)?,
        )
    }

    fn offset(self) -> Option<i16> {
        match self.zone {
            TimeZone::Utc => Some(0),
            TimeZone::Offset(minutes) => Some(minutes),
            TimeZone::Browser => None,
        }
    }
}

/// Defines the methods of a `Timestamp` on both `Expr` and `Sig`.
macro_rules! time_operations {
    ($ty:ident $(, $r:ident)?) => {
        impl<S $(, $r)?> $ty<S, Timestamp $(, $r)?> {
            /// The milliseconds since the Unix epoch.
            #[must_use]
            pub fn millis(&self) -> Expr<S, i64> {
                Expr::build(|scopes| self.operand().take(scopes))
            }

            /// The time `millis` milliseconds later, or earlier for a negative value.
            #[must_use]
            pub fn add_millis(&self, millis: impl IntoExpr<S, i64>) -> Expr<S, Timestamp> {
                Expr::build(|scopes| json!([
                    "add",
                    self.operand().take(scopes),
                    millis.into_expr().take(scopes)
                ]))
            }

            /// This time written in `format` with the browser's `Intl.DateTimeFormat`.
            ///
            /// ```
            /// use hypersig::{DateFormat, DateStyle, IntlLocale, Signals, TimeStyle, Timestamp};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Post {
            ///     posted: Timestamp,
            /// }
            ///
            /// const POSTED: DateFormat =
            ///     DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Short);
            /// let _label = Post::posted().format_date(&POSTED);
            /// ```
            #[must_use]
            pub fn format_date(&self, format: &DateFormat) -> Expr<S, String> {
                Expr::build(|scopes| json!([
                    "date",
                    self.operand().take(scopes),
                    format.locale.tag(),
                    format.date.map(DateStyle::name),
                    format.time.map(TimeStyle::name),
                    format.offset()
                ]))
            }

            /// This time relative to `now` in `locale`, such as `3 minutes ago`.
            ///
            /// The unit is the largest one the difference reaches: seconds
            /// under a minute, then minutes, hours, days under 30 days,
            /// months of 30 days under 365 days, and years of 365 days. The
            /// number is the difference in that unit, rounded toward zero,
            /// and the browser writes it with `Intl.RelativeTimeFormat`.
            ///
            /// The first HTML shows the text where the server knows `now`,
            /// as from a signal it fills with [`Timestamp::now`]. A page keeps
            /// the text current by setting that signal on a timer.
            ///
            /// ```
            /// use hypersig::{IntlLocale, Signals, Timestamp};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Chat {
            ///     sent: Timestamp,
            ///     now: Timestamp,
            /// }
            ///
            /// let _ago = Chat::sent().relative_to(Chat::now(), IntlLocale::En);
            /// ```
            #[must_use]
            pub fn relative_to(
                &self,
                now: impl IntoExpr<S, Timestamp>,
                locale: IntlLocale,
            ) -> Expr<S, String> {
                Expr::build(|scopes| json!([
                    "reltime",
                    self.operand().take(scopes),
                    now.into_expr().take(scopes),
                    locale.tag()
                ]))
            }
        }
    };
}

time_operations!(Expr);
time_operations!(Sig, R);

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS_EN: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
const WEEKDAYS_JA: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];

/// The milliseconds in a minute, which a [`TimeZone::Offset`] counts in.
const MILLIS_PER_MINUTE: i64 = 60_000;

/// The milliseconds in a day of UTC, which has no leap seconds in a [`Timestamp`].
const MILLIS_PER_DAY: i64 = 86_400_000;

/// The calendar date and the time of day of a moment, in UTC.
struct Civil {
    year: i64,
    /// 1 to 12.
    month: usize,
    day: i64,
    /// 0 for Sunday.
    weekday: usize,
    hour: i64,
    minute: i64,
    second: i64,
}

impl Civil {
    fn new(millis: i64) -> Option<Self> {
        let days = millis.div_euclid(MILLIS_PER_DAY);
        let seconds = millis.rem_euclid(MILLIS_PER_DAY) / 1000;
        // Howard Hinnant's `civil_from_days`, with 400-year eras that start on 1 March.
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let of_era = shifted.rem_euclid(146_097);
        let year_of_era = (of_era - of_era / 1460 + of_era / 36_524 - of_era / 146_096) / 365;
        let day_of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_index = (5 * day_of_year + 2) / 153;
        let month = if month_index < 10 {
            month_index + 3
        } else {
            month_index - 9
        };
        Some(Self {
            year: year_of_era + era * 400 + i64::from(month <= 2),
            month: usize::try_from(month).ok()?,
            day: day_of_year - (153 * month_index + 2) / 5 + 1,
            // 1 January 1970 was a Thursday.
            weekday: usize::try_from((days + 4).rem_euclid(7)).ok()?,
            hour: seconds / 3600,
            minute: seconds / 60 % 60,
            second: seconds % 60,
        })
    }
}

/// What `Intl.DateTimeFormat(locale, { dateStyle, timeStyle, timeZone: "UTC" })` writes for `millis`.
///
/// `None` where this crate has no rules: a locale other than `en` and `ja`,
/// and a year outside 1000 to 9999.
pub(crate) fn write_date(
    locale: &str,
    date: Option<&str>,
    time: Option<&str>,
    millis: i64,
) -> Option<String> {
    let at = Civil::new(millis)?;
    if !(1000..=9999).contains(&at.year) {
        return None;
    }
    let (year, month, day) = (at.year, at.month, at.day);
    let name = MONTHS.get(month - 1)?;
    let date_text = match (locale, date) {
        (_, None) => None,
        ("en", Some("full")) => Some(format!("{}, {name} {day}, {year}", WEEKDAYS_EN[at.weekday])),
        ("en", Some("long")) => Some(format!("{name} {day}, {year}")),
        ("en", Some("medium")) => Some(format!("{} {day}, {year}", name.get(..3)?)),
        ("en", Some("short")) => Some(format!("{month}/{day}/{:02}", year % 100)),
        ("ja", Some("full")) => Some(format!(
            "{year}年{month}月{day}日{}曜日",
            WEEKDAYS_JA[at.weekday]
        )),
        ("ja", Some("long")) => Some(format!("{year}年{month}月{day}日")),
        ("ja", Some("medium" | "short")) => Some(format!("{year}/{month:02}/{day:02}")),
        _ => return None,
    };
    let clock = match time {
        None => None,
        Some("medium") => Some(format!(":{:02}:{:02}", at.minute, at.second)),
        Some("short") => Some(format!(":{:02}", at.minute)),
        Some(_) => return None,
    };
    let time_text = clock.map(|clock| {
        if locale == "en" {
            let hour = if at.hour % 12 == 0 { 12 } else { at.hour % 12 };
            let half = if at.hour < 12 { "AM" } else { "PM" };
            format!("{hour}{clock} {half}")
        } else {
            format!("{}{clock}", at.hour)
        }
    });
    match (date_text, time_text) {
        (Some(date_text), Some(time_text)) => {
            let joiner = match (locale, date) {
                ("en", Some("full" | "long")) => " at ",
                ("en", _) => ", ",
                _ => " ",
            };
            Some(format!("{date_text}{joiner}{time_text}"))
        }
        (Some(text), None) | (None, Some(text)) => Some(text),
        (None, None) => None,
    }
}

/// The units `relative_to` picks from, with their length in milliseconds and their names.
const UNITS: [(f64, &str, &str, &str); 6] = [
    (31_536_000_000.0, "year", "years", "年"),
    (2_592_000_000.0, "month", "months", "か月"),
    (86_400_000.0, "day", "days", "日"),
    (3_600_000.0, "hour", "hours", "時間"),
    (60_000.0, "minute", "minutes", "分"),
    (1000.0, "second", "seconds", "秒"),
];

/// What `Intl.RelativeTimeFormat(locale).format` writes for `millis` from now, in the unit `reltime` in `runtime/src/eval.ts` picks.
#[expect(
    clippy::float_cmp,
    reason = "English uses the singular for exactly 1 after rounding toward zero"
)]
pub(crate) fn write_relative(locale: &str, millis: f64) -> Option<String> {
    let (length, one, many, ja) = UNITS
        .iter()
        .copied()
        .find(|(length, ..)| millis.abs() >= *length)
        .unwrap_or(UNITS[UNITS.len() - 1]);
    let count = (millis / length).trunc();
    let number = crate::ssr::format_number(count.abs(), 0, ",", ".", "", "");
    let past = count.is_sign_negative();
    Some(match locale {
        "en" => {
            let unit = if count.abs() == 1.0 { one } else { many };
            if past {
                format!("{number} {unit} ago")
            } else {
                format!("in {number} {unit}")
            }
        }
        "ja" => format!("{number} {ja}{}", if past { "前" } else { "後" }),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{CalendarDate, Timestamp, write_date, write_relative};

    /// The output of Node 22 with ICU 78 for each case.
    #[test]
    fn writes_dates_as_intl_does() {
        let at = 1_767_582_245_000; // 2026-01-05 03:04:05 UTC
        let write = |locale, date, time| write_date(locale, date, time, at).expect("in range");
        assert_eq!(
            write("en", Some("full"), Some("medium")),
            "Monday, January 5, 2026 at 3:04:05 AM"
        );
        assert_eq!(write("en", Some("long"), None), "January 5, 2026");
        assert_eq!(
            write("en", Some("medium"), Some("short")),
            "Jan 5, 2026, 3:04 AM"
        );
        assert_eq!(write("en", Some("short"), Some("short")), "1/5/26, 3:04 AM");
        assert_eq!(write("en", None, Some("medium")), "3:04:05 AM");
        assert_eq!(
            write("ja", Some("full"), Some("medium")),
            "2026年1月5日月曜日 3:04:05"
        );
        assert_eq!(write("ja", Some("long"), None), "2026年1月5日");
        assert_eq!(write("ja", Some("short"), Some("short")), "2026/01/05 3:04");
        let noon = 1_783_168_200_000; // 2026-07-04 12:30:00 UTC
        assert_eq!(
            write_date("en", Some("full"), Some("short"), noon).as_deref(),
            Some("Saturday, July 4, 2026 at 12:30 PM")
        );
        let midnight = 1_798_156_809_000; // 2026-12-25 00:00:09 UTC
        assert_eq!(
            write_date("en", Some("short"), Some("medium"), midnight).as_deref(),
            Some("12/25/26, 12:00:09 AM")
        );
        assert_eq!(
            write_date("ja", None, Some("short"), midnight).as_deref(),
            Some("0:00")
        );
        assert_eq!(
            write_date("en", Some("long"), None, -1).as_deref(),
            Some("December 31, 1969")
        );
        assert_eq!(
            write_date("en", Some("long"), None, -62_135_596_800_001),
            None
        );
    }

    #[test]
    fn writes_relative_times_as_intl_does() {
        assert_eq!(
            write_relative("en", -180_000.0).as_deref(),
            Some("3 minutes ago")
        );
        assert_eq!(
            write_relative("en", 3_600_000.0).as_deref(),
            Some("in 1 hour")
        );
        assert_eq!(
            write_relative("en", -500.0).as_deref(),
            Some("0 seconds ago")
        );
        assert_eq!(write_relative("en", 0.0).as_deref(), Some("in 0 seconds"));
        assert_eq!(
            write_relative("en", -1234.0 * 31_536_000_000.0).as_deref(),
            Some("1,234 years ago")
        );
        assert_eq!(
            write_relative("ja", -10_800_000.0).as_deref(),
            Some("3 時間前")
        );
        assert_eq!(
            write_relative("ja", 2.0 * 2_592_000_000.0).as_deref(),
            Some("2 か月後")
        );
        assert_eq!(
            write_relative("ja", 259_200_000.0).as_deref(),
            Some("3 日後")
        );
    }

    #[test]
    fn rounds_a_system_time_down_to_the_millisecond() {
        let before = std::time::UNIX_EPOCH - std::time::Duration::from_micros(1500);
        assert_eq!(Timestamp::from(before).as_millis(), -2);
        let after = std::time::UNIX_EPOCH + std::time::Duration::from_micros(1500);
        assert_eq!(Timestamp::from(after).as_millis(), 1);
    }

    #[test]
    fn reads_and_writes_calendar_dates() {
        let leap = CalendarDate::parse("2024-02-29").expect("2024 is a leap year");
        assert_eq!((leap.year(), leap.month(), leap.day()), (2024, 2, 29));
        assert_eq!(CalendarDate::new(1, 1, 1).to_string(), "0001-01-01");
        for text in [
            "1900-02-29",
            "2026-13-01",
            "2026-04-31",
            "0000-01-01",
            "2026-1-05",
            "2026/01/05",
            "+2026-01-05",
            "",
        ] {
            assert!(CalendarDate::parse(text).is_err(), "{text}");
        }
        let json = serde_json::to_string(&CalendarDate::new(2026, 1, 5)).expect("serializes");
        assert_eq!(json, r#""2026-01-05""#);
        serde_json::from_str::<CalendarDate>(r#""2026-02-30""#).unwrap_err();
        let value = serde_json::json!("2000-02-29");
        assert_eq!(
            serde_json::from_value::<CalendarDate>(value).ok(),
            Some(CalendarDate::new(2000, 2, 29))
        );
    }
}
