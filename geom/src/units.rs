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

// In seconds. Can be negative.
// TODO Naming is awkward. Can represent a moment in time or a duration.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Duration(f64);

impl Duration {
    pub const ZERO: Duration = Duration::const_seconds(0.0);

    pub fn seconds(value: f64) -> Duration {
        if !value.is_finite() {
            panic!("Bad Duration {}", value);
        }

        Duration(value)
    }

    pub const fn const_seconds(value: f64) -> Duration {
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
        self.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl ops::Sub for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration::seconds(self.0 - other.0)
    }
}

impl ops::Mul<f64> for Duration {
    type Output = Duration;

    fn mul(self, other: f64) -> Duration {
        Duration::seconds(self.0 * other)
    }
}

impl ops::Mul<Speed> for Duration {
    type Output = Distance;

    fn mul(self, other: Speed) -> Distance {
        Distance::meters(self.0 * other.0)
    }
}

impl ops::Div<Duration> for Duration {
    type Output = f64;

    fn div(self, other: Duration) -> f64 {
        if other.0 == 0.0 {
            panic!("Can't divide {} / {}", self, other);
        }
        self.0 / other.0
    }
}

// In meters per second. Can be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Speed(f64);

impl Speed {
    pub const ZERO: Speed = Speed::const_meters_per_second(0.0);

    pub fn meters_per_second(value: f64) -> Speed {
        if !value.is_finite() {
            panic!("Bad Speed {}", value);
        }

        Speed(value)
    }

    pub const fn const_meters_per_second(value: f64) -> Speed {
        Speed(value)
    }

    pub fn miles_per_hour(value: f64) -> Speed {
        Speed::meters_per_second(0.44704 * value)
    }

    // TODO Remove if possible.
    pub fn inner_meters_per_second(self) -> f64 {
        self.0
    }
}

impl ops::Add for Speed {
    type Output = Speed;

    fn add(self, other: Speed) -> Speed {
        Speed::meters_per_second(self.0 + other.0)
    }
}

impl ops::Sub for Speed {
    type Output = Speed;

    fn sub(self, other: Speed) -> Speed {
        Speed::meters_per_second(self.0 - other.0)
    }
}

impl ops::Mul<f64> for Speed {
    type Output = Speed;

    fn mul(self, scalar: f64) -> Speed {
        Speed::meters_per_second(self.0 * scalar)
    }
}

impl ops::Mul<Duration> for Speed {
    type Output = Distance;

    fn mul(self, other: Duration) -> Distance {
        Distance::meters(self.0 * other.0)
    }
}

impl ops::Div<Duration> for Speed {
    type Output = Acceleration;

    fn div(self, other: Duration) -> Acceleration {
        if other == Duration::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        Acceleration::meters_per_second_squared(self.0 / other.0)
    }
}

impl ops::Div<Acceleration> for Speed {
    type Output = Duration;

    fn div(self, other: Acceleration) -> Duration {
        if other == Acceleration::ZERO {
            panic!("Can't divide {} / {}", self, other);
        }
        Duration::seconds(self.0 / other.0)
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m/s", self.0)
    }
}

// In meters per second^2. Can be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Acceleration(f64);

impl Acceleration {
    pub const ZERO: Acceleration = Acceleration::const_meters_per_second_squared(0.0);

    pub fn meters_per_second_squared(value: f64) -> Acceleration {
        if !value.is_finite() {
            panic!("Bad Acceleration {}", value);
        }

        Acceleration(value)
    }

    pub const fn const_meters_per_second_squared(value: f64) -> Acceleration {
        Acceleration(value)
    }

    // TODO Remove by making Acceleration itself Ord.
    pub fn as_ordered(self) -> NotNan<f64> {
        NotNan::new(self.0).unwrap()
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
        self.0
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m/s^2", self.0)
    }
}

impl ops::Mul<Duration> for Acceleration {
    type Output = Speed;

    fn mul(self, other: Duration) -> Speed {
        Speed::meters_per_second(self.0 * other.0)
    }
}
