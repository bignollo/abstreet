use ordered_float::NotNan;
use serde_derive::{Deserialize, Serialize};
use std::{fmt, i32, ops};

// Centimeter resolution, can be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Distance(i32);

impl Distance {
    pub const ZERO: Distance = Distance::const_cm(0);
    pub const MAX: Distance = Distance::const_cm(i32::MAX);
    pub(crate) const EPSILON_METERS: f64 = 0.01;

    pub fn meters(value: f64) -> Distance {
        if !value.is_finite() {
            panic!("Bad Distance {}", value);
        }

        Distance((value * 100.0).round() as i32)
    }

    // TODO Ideally would have const_meters.
    pub const fn const_cm(value: i32) -> Distance {
        Distance(value)
    }

    pub fn inches(value: f64) -> Distance {
        Distance::meters(0.0254 * value)
    }

    pub fn abs(self) -> Distance {
        if self.0 >= 0 {
            self
        } else {
            self * -1.0
        }
    }

    pub fn sqrt(self) -> Distance {
        Distance::meters(self.inner_meters().sqrt())
    }

    // TODO Remove by making Distance itself Ord.
    pub fn as_ordered(self) -> NotNan<f64> {
        NotNan::new(self.inner_meters()).unwrap()
    }

    // TODO Remove if possible.
    pub fn inner_meters(self) -> f64 {
        f64::from(self.0) / 100.0
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO commas every third place
        write!(f, "{}m", self.inner_meters())
    }
}

impl ops::Add for Distance {
    type Output = Distance;

    fn add(self, other: Distance) -> Distance {
        Distance(self.0.checked_add(other.0).unwrap())
    }
}

impl ops::AddAssign for Distance {
    fn add_assign(&mut self, other: Distance) {
        *self = *self + other;
    }
}

impl ops::Sub for Distance {
    type Output = Distance;

    fn sub(self, other: Distance) -> Distance {
        Distance(self.0.checked_sub(other.0).unwrap())
    }
}

impl ops::SubAssign for Distance {
    fn sub_assign(&mut self, other: Distance) {
        *self = *self - other;
    }
}

impl ops::Neg for Distance {
    type Output = Distance;

    fn neg(self) -> Distance {
        Distance(-self.0)
    }
}

impl ops::Mul<f64> for Distance {
    type Output = Distance;

    fn mul(self, scalar: f64) -> Distance {
        Distance::meters(self.inner_meters() * scalar)
    }
}

impl ops::Div<Distance> for Distance {
    type Output = f64;

    fn div(self, other: Distance) -> f64 {
        if other == Distance::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        self.inner_meters() / other.inner_meters()
    }
}

impl ops::Div<f64> for Distance {
    type Output = Distance;

    fn div(self, scalar: f64) -> Distance {
        if scalar == 0.0 {
            panic!("Can't divide {} / {}", self, scalar);
        }
        Distance::meters(self.inner_meters() / scalar)
    }
}

// Millisecond resolution. Can be negative.
// TODO Naming is awkward. Can represent a moment in time or a duration.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Duration(i32);

impl Duration {
    pub const ZERO: Duration = Duration::const_ms(0);

    pub fn seconds(value: f64) -> Duration {
        if !value.is_finite() {
            panic!("Bad Duration {}", value);
        }

        Duration((value * 1000.0).round() as i32)
    }

    pub const fn const_ms(value: i32) -> Duration {
        Duration(value)
    }

    pub fn min(self, other: Duration) -> Duration {
        if self <= other {
            self
        } else {
            other
        }
    }

    // TODO Remove if possible.
    pub fn inner_seconds(self) -> f64 {
        f64::from(self.0) / 1000.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.inner_seconds())
    }
}

impl ops::Sub for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration(self.0.checked_sub(other.0).unwrap())
    }
}

impl ops::Mul<f64> for Duration {
    type Output = Duration;

    fn mul(self, scalar: f64) -> Duration {
        Duration::seconds(self.inner_seconds() * scalar)
    }
}

impl ops::Mul<Speed> for Duration {
    type Output = Distance;

    fn mul(self, other: Speed) -> Distance {
        Distance::meters(self.inner_seconds() * other.inner_meters_per_second())
    }
}

impl ops::Div<Duration> for Duration {
    type Output = f64;

    fn div(self, other: Duration) -> f64 {
        if other == Duration::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        self.inner_seconds() / other.inner_seconds()
    }
}

// In 10^-4 meters per second. Can be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Speed(i32);

impl Speed {
    pub const ZERO: Speed = Speed::const_sorta_meters_per_second(0);

    pub fn meters_per_second(value: f64) -> Speed {
        if !value.is_finite() {
            panic!("Bad Speed {}", value);
        }

        Speed((value * 10_000.0).round() as i32)
    }

    // TODO Funky name. Meters per second, but multiply by 10^4.
    pub const fn const_sorta_meters_per_second(value: i32) -> Speed {
        Speed(value)
    }

    pub fn miles_per_hour(value: f64) -> Speed {
        Speed::meters_per_second(0.44704 * value)
    }

    // TODO Remove if possible.
    pub fn inner_meters_per_second(self) -> f64 {
        f64::from(self.0) / 10_000.0
    }
}

impl ops::Add for Speed {
    type Output = Speed;

    fn add(self, other: Speed) -> Speed {
        Speed(self.0.checked_add(other.0).unwrap())
    }
}

impl ops::Sub for Speed {
    type Output = Speed;

    fn sub(self, other: Speed) -> Speed {
        Speed(self.0.checked_sub(other.0).unwrap())
    }
}

impl ops::Mul<f64> for Speed {
    type Output = Speed;

    fn mul(self, scalar: f64) -> Speed {
        Speed::meters_per_second(self.inner_meters_per_second() * scalar)
    }
}

impl ops::Mul<Duration> for Speed {
    type Output = Distance;

    fn mul(self, other: Duration) -> Distance {
        Distance::meters(self.inner_meters_per_second() * other.inner_seconds())
    }
}

impl ops::Div<Duration> for Speed {
    type Output = Acceleration;

    fn div(self, other: Duration) -> Acceleration {
        if other == Duration::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        Acceleration::meters_per_second_squared(
            self.inner_meters_per_second() / other.inner_seconds(),
        )
    }
}

impl ops::Div<Acceleration> for Speed {
    type Output = Duration;

    fn div(self, other: Acceleration) -> Duration {
        if other == Acceleration::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        Duration::seconds(self.inner_meters_per_second() / other.inner_meters_per_second_squared())
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m/s", self.inner_meters_per_second())
    }
}

// In 10^-4 meters per second^2. Can be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Acceleration(i32);

impl Acceleration {
    pub const ZERO: Acceleration = Acceleration(0);

    pub fn meters_per_second_squared(value: f64) -> Acceleration {
        if !value.is_finite() {
            panic!("Bad Acceleration {}", value);
        }

        Acceleration((value * 10_000.0).round() as i32)
    }

    // TODO Remove by making Acceleration itself Ord.
    pub fn as_ordered(self) -> NotNan<f64> {
        NotNan::new(self.inner_meters_per_second_squared()).unwrap()
    }

    pub fn min(self, other: Acceleration) -> Acceleration {
        if self <= other {
            self
        } else {
            other
        }
    }

    // TODO Remove if possible.
    pub fn inner_meters_per_second_squared(self) -> f64 {
        f64::from(self.0) / 10_000.0
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m/s^2", self.inner_meters_per_second_squared())
    }
}

impl ops::Mul<Duration> for Acceleration {
    type Output = Speed;

    fn mul(self, other: Duration) -> Speed {
        Speed::meters_per_second(self.inner_meters_per_second_squared() * other.inner_seconds())
    }
}
