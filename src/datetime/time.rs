use datetime::date::Date;
use time::{Tm, now_utc, now};

#[derive(Clone)]
pub struct Time {
    pub date: Date,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub nanos: u32,
}

impl Time {
    pub fn from_tm(tm: Tm) -> Time {
        let year = tm.tm_year + 1900;
        let month = tm.tm_mon + 1;

        let date = Date {
            year: year as u32,
            month: month as u32,
            day: tm.tm_mday as u32,
        };

        Time {
            date: date,
            hours: tm.tm_hour as u32,
            minutes: tm.tm_min as u32,
            seconds: tm.tm_sec as u32,
            nanos: tm.tm_nsec as u32,
        }
    }

    pub fn new() -> Time {
        let date = Date {
            year: 0,
            month: 0,
            day: 0,
        };
        Time {
            date: date,
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanos: 0,
        }
    }

    pub fn now() -> Time {
        Time::from_tm(now())
    }

    pub fn now_utc() -> Time {
        Time::from_tm(now_utc())
    }

    pub fn with_date(date: Date) -> Time {
        Time {
            date: date,
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanos: 0,
        }
    }

    pub fn year(mut self, year: u32) -> Time {
        self.date.year = year;
        self
    }

    pub fn month(mut self, month: u32) -> Time {
        self.date.month = month;
        self
    }

    pub fn day(mut self, day: u32) -> Time {
        self.date.day = day;
        self
    }

    pub fn hour(mut self, hour: u32) -> Time {
        self.hours = hour;
        self
    }

    pub fn minute(mut self, minute: u32) -> Time {
        self.minutes = minute;
        self
    }

    pub fn second(mut self, second: u32) -> Time {
        self.seconds = second;
        self
    }

    pub fn nanos(mut self, nanos: u32) -> Time {
        self.nanos = nanos;
        self
    }
}
