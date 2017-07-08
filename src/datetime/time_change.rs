pub use time::Tm;
pub use time::now_utc;
pub use datetime::date::Date;
pub use datetime::time::Time;
pub use datetime::period::Period;

pub struct TimeChange {
    years: Option<f32>,
    months: Option<f32>,
    weeks: Option<f32>,
    days: Option<f32>,
    hours: Option<f32>,
    minutes: Option<f32>,
    seconds: Option<f32>,
    nanos: Option<u64>,
}

impl TimeChange {
    pub fn new() -> TimeChange {
        TimeChange {
            years: None,
            months: None,
            weeks: None,
            days: None,
            hours: None,
            minutes: None,
            seconds: None,
            nanos: None,
        }
    }

    pub fn from_now(&self) -> Time {
        let time = TimeChange {
            years: self.years,
            months: self.months,
            weeks: self.weeks,
            days: self.days,
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
            nanos: self.nanos,
        };

        Time::now().advance(time)
    }

    pub fn years(mut self, years: f32) -> TimeChange {
        self.years = Some(years);
        self
    }

    pub fn months(mut self, months: f32) -> TimeChange {
        self.months = Some(months);
        self
    }

    pub fn weeks(mut self, weeks: f32) -> TimeChange {
        self.weeks = Some(weeks);
        self
    }

    pub fn days(mut self, days: f32) -> TimeChange {
        self.days = Some(days);
        self
    }

    pub fn hours(mut self, hours: f32) -> TimeChange {
        self.hours = Some(hours);
        self
    }

    pub fn minutes(mut self, minutes: f32) -> TimeChange {
        self.minutes = Some(minutes);
        self
    }

    pub fn seconds(mut self, seconds: f32) -> TimeChange {
        self.seconds = Some(seconds);
        self
    }
}

trait ChangeTime {
    fn advance(&self, change: TimeChange) -> Time;
}

impl ChangeTime for Time {
    fn advance(&self, change: TimeChange) -> Time {
        let mut years = None;
        let mut months = None;
        let mut days = change.days;
        let mut hours = change.hours;
        let mut date = &self.date;

        change.years.map(|supplied_years| {
            let full_years = supplied_years.trunc();
            years = Some(full_years);
            let partial_years = supplied_years - full_years;

            days = Some((partial_years * 365f32) + days.unwrap_or(0f32));
        });

        change.months.map(|supplied_months| {
            let full_months = supplied_months.trunc();
            months = Some(full_months);
            let partial_months = supplied_months - full_months;

            days = Some((partial_months * 30f32) + days.unwrap_or(0f32));
        });

        change.weeks.map(|supplied_weeks| {
            days = Some(supplied_weeks * 7f32 + days.unwrap_or(0f32));
        });

        days.map(|supplied_days| {
            let full_days = supplied_days.trunc();
            days = Some(full_days);
            let partial_days = supplied_days - full_days;

            hours = Some((partial_days * 24f32) + hours.unwrap_or(0f32));
        });

        // TODO: years, months, weeks

        let mut carry_minutes = 0f32;
        let mut carry_hours = 0f32;
        let mut carry_days = 0f32;
        let mut changed_seconds = self.seconds as f32;
        let mut changed_minutes = self.minutes as f32;
        let mut changed_hours = self.hours as f32;

        change.seconds.map(|seconds| {
            if seconds + changed_seconds > 59f32 {
                carry_minutes = 1f32;
                changed_seconds += seconds - 60f32;
            }
        });

        change.minutes.map(|minutes| {
            if minutes + changed_minutes + carry_minutes > 59f32 {
                carry_hours = 1f32;
                changed_minutes += minutes + carry_minutes - 60f32;
            }
        });

        change.hours.map(|hours| {
            if hours + changed_hours + carry_hours > 23f32 {
                carry_days = 1f32;
                changed_hours += hours + carry_hours - 24f32;
            }
        });

        days = Some(days.unwrap_or(0f32) + carry_days);



        years.map(|years| date = &date.advance_years(years as u32));
        months.map(|months| date = &date.advance_months(months as i32));
        days.map(|days| date = &date.advance_days(days as u32));

        let time = Time::with_date(date)
            .hour(changed_hours as u32)
            .minute(changed_minutes as u32)
            .second(changed_seconds as u32)
            .nanos(self.nanos);

        time
    }
}
