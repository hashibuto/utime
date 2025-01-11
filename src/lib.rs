use std::{
    num::{ParseFloatError, ParseIntError},
    time::{SystemTime, UNIX_EPOCH},
};

const EPOCH_YEAR: u64 = 1970;
const DAYS_OF_MONTH_LEAP: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const MONTH_DAYS_LEAP: [u64; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
const DAYS_OF_MONTH_NOLEAP: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const MONTH_DAYS_NOLEAP: [u64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

const MICROS_PER_SECOND: u64 = 1000000;
const MICROS_PER_MINUTE: u64 = MICROS_PER_SECOND * 60;
const MICROS_PER_HOUR: u64 = MICROS_PER_MINUTE * 60;
const MICROS_PER_DAY: u64 = MICROS_PER_HOUR * 24;

pub struct Utime {
    timestamp: u64,
}

#[derive(Debug)]
pub struct Error {}

impl Error {
    fn new() -> Self {
        Self {}
    }
}

macro_rules! leap_year {
    ($year:expr) => {
        $year % 4 == 0 && ($year % 100 != 0 || $year % 400 == 0)
    };
}

macro_rules! year_days {
    ($is_leap:expr) => {
        match $is_leap {
            true => 366,
            false => 365,
        }
    };
}

impl Utime {
    pub fn new() -> Self {
        Self { timestamp: 0 }
    }

    pub fn from_date_time(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
        microseconds: u64,
    ) -> Result<Self, Error> {
        if year < EPOCH_YEAR {
            return Err(Error::new());
        }
        if month < 1 || month > 12 {
            return Err(Error::new());
        }
        if hour > 23 {
            return Err(Error::new());
        }
        if minute > 59 {
            return Err(Error::new());
        }
        if second > 59 {
            return Err(Error::new());
        }
        if microseconds >= MICROS_PER_SECOND {
            return Err(Error::new());
        }

        let is_leap = leap_year!(year);
        let dom: &[u64];
        let dom_cum: &[u64];
        if is_leap {
            dom = &DAYS_OF_MONTH_LEAP;
            dom_cum = &MONTH_DAYS_LEAP;
        } else {
            dom = &DAYS_OF_MONTH_NOLEAP;
            dom_cum = &MONTH_DAYS_NOLEAP;
        }

        let max_days = dom[(month - 1) as usize];
        if day < 1 || day > max_days {
            return Err(Error::new());
        }
        let month_days: u64 = dom_cum[(month - 1) as usize];

        // first calculate the number of non-leap year centuries in the date range
        let mut non_leap = 0;
        for y in (2000..year).step_by(100) {
            if !leap_year!(y) {
                non_leap += 1;
            }
        }

        let leap_years = (year - 1969) / 4;
        let year_days = ((year - 1970) * 365) + leap_years - non_leap;

        return Ok(Self {
            timestamp: ((year_days + month_days + (day - 1)) * MICROS_PER_DAY)
                + (hour * MICROS_PER_HOUR)
                + (minute * MICROS_PER_MINUTE)
                + (second * MICROS_PER_SECOND)
                + microseconds,
        });
    }

    pub fn from_iso_3601_datetime(dt: String) -> Result<Self, Error> {
        let res = dt.split_once("T");
        if res.is_none() {
            return Err(Error::new());
        }

        let (dt_str, time_str) = res.unwrap();
        let date_parts = dt_str.split("-").collect::<Vec<&str>>();
        if date_parts.len() != 3 {
            return Err(Error::new());
        }

        let year_res: Result<u64, ParseIntError> = date_parts[0].parse();
        if year_res.is_err() {
            return Err(Error::new());
        }
        let year = year_res.unwrap();

        let month_res: Result<u64, ParseIntError> = date_parts[1].parse();
        if month_res.is_err() {
            return Err(Error::new());
        }
        let month = month_res.unwrap();

        let day_res: Result<u64, ParseIntError> = date_parts[2].parse();
        if day_res.is_err() {
            return Err(Error::new());
        }
        let day = day_res.unwrap();

        let time_parts = time_str.split(":").collect::<Vec<&str>>();
        if time_parts.len() != 3 {
            return Err(Error::new());
        }

        let hour_res: Result<u64, ParseIntError> = time_parts[0].parse();
        if hour_res.is_err() {
            return Err(Error::new());
        }
        let hour = hour_res.unwrap();

        let minute_res: Result<u64, ParseIntError> = time_parts[1].parse();
        if minute_res.is_err() {
            return Err(Error::new());
        }
        let minute = minute_res.unwrap();

        let sec_opt = time_parts[2].strip_suffix("Z");
        if sec_opt.is_none() {
            return Err(Error::new());
        }

        let sec_str = sec_opt.unwrap();
        let sec_res: Result<f64, ParseFloatError> = sec_str.parse();
        if sec_res.is_err() {
            return Err(Error::new());
        }
        let sec = sec_res.unwrap();
        let full_sec: u64 = sec as u64;
        let micros = (sec - (full_sec as f64)) * 1000000.0;
        return Utime::from_date_time(year, month, day, hour, minute, full_sec, micros as u64);
    }

    pub fn now() -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        }
    }

    pub fn is_zero(&self) -> bool {
        return self.timestamp == 0;
    }

    pub fn as_micros(&self) -> u64 {
        return self.timestamp;
    }

    pub fn as_milis(&self) -> u64 {
        return self.timestamp / 1000;
    }

    pub fn as_seconds(&self) -> u64 {
        return self.timestamp / 1000000;
    }

    pub fn as_iso_8601_datetime(&self) -> String {
        let (year, month, day, hour, minute, second, microsecond) = self.to_components();
        let sec: f64 = second as f64 + (microsecond as f64 / 1000000.0);
        return format!(
            "{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>6.3}Z",
            year, month, day, hour, minute, sec
        );
    }

    pub fn as_iso_8601_date(&self) -> String {
        let (year, month, day, hour, minute, second, microsecond) = self.to_components();
        return format!("{}-{:0>2}-{:0>2}", year, month, day);
    }

    pub fn to_components(&self) -> (u64, u64, u64, u64, u64, u64, u64) {
        let mut leftover = self.timestamp;
        let mut total_days = leftover / MICROS_PER_DAY;
        leftover -= total_days * MICROS_PER_DAY;
        let total_hours = leftover / MICROS_PER_HOUR;
        leftover -= total_hours * MICROS_PER_HOUR;
        let total_minutes = leftover / MICROS_PER_MINUTE;
        leftover -= total_minutes * MICROS_PER_MINUTE;
        let total_seconds = leftover / MICROS_PER_SECOND;
        leftover -= total_seconds * MICROS_PER_SECOND;

        let mut min_years = (total_days / 366) + 1970;
        // first calculate the number of non-leap year centuries in the date range
        let mut non_leap: u64 = 0;
        for y in (2000..min_years).step_by(100) {
            if !leap_year!(y) {
                non_leap += 1;
            }
        }

        let leap_years = (min_years - 1969) / 4;
        let year_days = ((min_years - 1970) * 365) + leap_years - non_leap;

        total_days -= year_days;
        loop {
            let d: u64 = year_days!(leap_year!(min_years + 1));
            if total_days >= d {
                total_days -= d;
                min_years += 1;
            } else {
                break;
            }
        }

        let month = total_days / 28;
        let is_leap = leap_year!(min_years);
        let dom_cum: &[u64];
        if is_leap {
            dom_cum = &MONTH_DAYS_LEAP;
        } else {
            dom_cum = &MONTH_DAYS_NOLEAP;
        }
        total_days -= dom_cum[month as usize];

        return (
            min_years,
            month + 1,
            total_days + 1,
            total_hours,
            total_minutes,
            total_seconds,
            leftover,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year_macro() {
        assert!(!leap_year!(1970));
        assert!(leap_year!(1972));
        assert!(leap_year!(1992));
        assert!(!leap_year!(1994));
        assert!(leap_year!(2000));
        assert!(leap_year!(2088));
        assert!(!leap_year!(2100))
    }

    #[test]
    fn test_timestamp_from_date_1() {
        let t = Utime::from_date_time(1970, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 0);
    }

    #[test]
    fn test_timestamp_from_date_2() {
        let t = Utime::from_date_time(1971, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 31536000);
    }

    #[test]
    fn test_timestamp_from_date_3() {
        let t = Utime::from_date_time(1972, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 63072000);
    }

    #[test]
    fn test_timestamp_from_date_4() {
        let t = Utime::from_date_time(1973, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 94694400);
    }

    #[test]
    fn test_timestamp_from_date_5() {
        let t = Utime::from_date_time(1974, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 126230400);
    }

    #[test]
    fn test_timestamp_from_date_6() {
        let t = Utime::from_date_time(1976, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 189302400);
    }

    #[test]
    fn test_timestamp_from_date_century() {
        let t = Utime::from_date_time(2000, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 946684800);
    }

    #[test]
    fn test_timestamp_from_date_century_1() {
        let t = Utime::from_date_time(2002, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 1009843200);
    }

    #[test]
    fn test_timestamp_from_date_century_2() {
        let t = Utime::from_date_time(2150, 1, 1, 0, 0, 0, 0).unwrap();
        assert!(t.as_seconds() == 5680281600);
    }

    #[test]
    fn test_timestamp_from_date_time() {
        let t = Utime::from_date_time(2150, 2, 2, 23, 11, 30, 0).unwrap();
        assert!(t.as_seconds() == 5683129890);
    }

    #[test]
    fn test_get_components() {
        let t = Utime::from_date_time(2150, 2, 2, 23, 11, 30, 0).unwrap();
        let (year, month, day, hour, minute, second, microsecond) = t.to_components();
        assert!(year == 2150);
        assert!(month == 2);
        assert!(day == 2);
        assert!(hour == 23);
        assert!(minute == 11);
        assert!(second == 30);
        assert!(microsecond == 0);
    }

    #[test]
    fn test_get_components_1() {
        let t = Utime::from_date_time(1971, 1, 1, 0, 11, 30, 0).unwrap();
        let (year, month, day, hour, minute, second, microsecond) = t.to_components();
        assert!(year == 1971);
        assert!(month == 1);
        assert!(day == 1);
        assert!(hour == 0);
        assert!(minute == 11);
        assert!(second == 30);
        assert!(microsecond == 0);
    }

    #[test]
    fn test_iso_8601_datetime() {
        let t = Utime::from_date_time(2150, 2, 2, 3, 1, 5, 30000).unwrap();
        let tstr = t.as_iso_8601_datetime();
        assert!(tstr == "2150-02-02T03:01:05.030Z")
    }

    #[test]
    fn test_iso_8601_date() {
        let t = Utime::from_date_time(2150, 2, 2, 3, 1, 5, 30000).unwrap();
        let tstr = t.as_iso_8601_date();
        assert!(tstr == "2150-02-02")
    }

    #[test]
    fn test_parse_iso_8601_datetime() {
        let t = Utime::from_date_time(2150, 2, 2, 3, 1, 5, 30000).unwrap();
        let tstr = t.as_iso_8601_datetime();
        assert!(tstr == "2150-02-02T03:01:05.030Z");
        let t1 = Utime::from_iso_3601_datetime(tstr).unwrap();
        let (year, month, day, hour, minute, second, microsecond) = t1.to_components();
        assert!(year == 2150);
        assert!(month == 2);
        assert!(day == 2);
        assert!(hour == 3);
        assert!(minute == 1);
        assert!(second == 5);
        assert!(microsecond == 30000);
    }
}
