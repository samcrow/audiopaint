use std::cmp::Ordering;

/// Stores the amplitude of a frequency component
#[derive(Debug,Copy,Clone)]
pub struct Amplitude {
    /// The amplitude, in the range [0, 1]
    value: f64,
}

impl Amplitude {
    /// Creates a new amplitude value
    /// If the value is outside the range [0, 1], it will be clamped.
    /// If the value is NaN, it will be set to 0.
    pub fn from(value: f64) -> Amplitude {
        let mut value = value;
        if value.is_nan() {
            value = 0f64;
        }
        if value > 1f64 {
            value = 1f64;
        }
        else if value < 0f64 {
            value = 0f64;
        }
        Amplitude { value: value }
    }

    /// Returns the value of this amplitude
    pub fn value(self) -> f64 {
        self.value
    }
}

/// Stores the value of an audio signal at a point in time
#[derive(Debug,Copy,Clone)]
pub struct Value {
    /// The amplitude, in the range [-1, 1]
    value: f64,
}

impl Value {
    /// Creates a new amplitude value
    /// If the value is NaN, it will be set to 0.
    pub fn from(value: f64) -> Value {
        let mut value = value;
        if value.is_nan() {
            value = 0f64;
        }
        Value { value: value }
    }

    /// Returns the value of this amplitude
    pub fn value(self) -> f64 {
        self.value
    }
    /// Returns the absolute value of this value
    pub fn abs(self) -> Value {
        Value { value: self.value.abs() }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.value.eq(&other.value)
    }
}
impl Eq for Value {}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
