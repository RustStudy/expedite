use time::{Tm};
use datetime::time_change::TimeChange;

#[derive(Clone, Debug)]
#[derive(Eq, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

// Magic variables from http://en.wikipedia.org/wiki/Julian_day#Gregorian_calendar_from_Julian_day_number

static Y: u32 = 4716;
static J: u32 = 1401;
static M: u32 = 2;
static N: u32 = 12;
static R: u32 = 4;
static P: u32 = 1461;
static V: u32 = 3;
static U: u32 = 5;
static S: u32 = 153;
static W: u32 = 2;
static B: u32 = 274277;
static C: u32 = 38;

impl Date {
    fn from_jdn(jdn: u32) -> Date {
        let f = jdn + J + (((4u32.wrapping_mul(jdn) + B) / 146097) * 3) / 4 - C;
        let e = R.wrapping_mul(f) + V;
        let g = (e % P) / R;
        let h = U * g + W;
        let day = (h % S) / U + 1;
        let month = ((h / S + M) % N) + 1;
        let year = e / P - Y + (N + M - month) / N;

        Date {
            day: day,
            month: month,
            year: year as u32,
        }
    }

    pub fn new(year: u32, month: u32, day: u32) -> Date {
        Date {
            day: day,
            month: month,
            year: year,
        }
    }

    pub fn advance_days(&self, n: u32) -> Date {
        let jdn = self.to_jdn() as u32;
        Date::from_jdn((jdn + n) as u32)
    }

    pub fn advance_years(&self, n: u32) -> Date {
        Date::new(self.year + n, self.month, self.day)
    }

    pub fn advance_months(&self, n: i32) -> Date {
        if (n >= 0) {
            self.add_months(n as u32)
        } else {
            self.subtract_months(-n as u32)
        }
    }

    fn add_months(&self, n: u32) -> Date {
        let remaining_in_year = 12 - self.month;
        let after_this_year = n.wrapping_sub(remaining_in_year);

        if (after_this_year > 0) {
            let years = after_this_year / 12 + 1;
            let months = after_this_year % 12;

            Date::new(self.year + years as u32, months, self.day)
        } else {
            Date::new(self.year, self.month + n, self.day)
        }
    }

    fn subtract_months(&self, n: u32) -> Date {
        if (n > self.month) {
            let years = n / 12;
            let remaining_in_year = n % 12;
            Date::new(self.year - years as u32,
                      12 - remaining_in_year + self.month,
                      self.day)
        } else {
            Date::new(self.year, self.month - n, self.day)
        }
    }

    pub fn to_jdn(&self) -> u32 {
        let month = self.month;

        let a = (14 - self.month) / 12;    // 1 for Jan/Feb, 0 otherwise
        let y: u32 = (self.year as u32) + 4800 - a;
        let m = self.month + (12 * a) - 3; // 0 for Mar, 11 for Feb

        let mut jdn: u32 = self.day;
        jdn += ((153 * m) + 2) / 5;
        jdn += (365u32.wrapping_mul(y));
        jdn += (y / 4);
        jdn -= (y / 100);
        jdn += (y / 400);
        jdn -= 32045;

        jdn as u32
    }
}

pub trait ToDate {
    fn to_date(&self) -> Date;
}

impl ToDate for Tm {
    fn to_date(&self) -> Date {
        Date {
            day: self.tm_mday as u32,
            month: self.tm_mon as u32,
            year: self.tm_year as u32,
        }
    }
}
