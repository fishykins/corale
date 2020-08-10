use num::{Float, Integer, FromPrimitive};

pub fn clamp<T: Float>(min: T, max: T, value: T) -> T {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    value
}

pub fn clamp_int<T: Integer>(min: T, max: T, value: T) -> T {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    value
}

/// Lerp between a and b by amount (0 - 1)
pub fn lerp<T: Float>(a: T, b: T, amount: T) -> T {
    if a == b {
        return a;
    }
    if a > b {
        let range: T = a - b;
        return b + ( range * amount);
    } else {
        let range = b - a;
        return a + ( range * amount);
    }
}

/// Lerp between a and b by amount (0 - 1)
pub fn lerp_int<T>(a: T, b: T, percent: T) -> T
    where T: Integer + FromPrimitive + Copy
{
    if a == b {
        return a;
    }
    if a > b {
        let range: T = a - b;
        return b + ( range * range / T::from_usize(100).unwrap() * percent);
    } else {
        let range = b - a;
        return a + ( range * range / T::from_usize(100).unwrap() * percent);
    }
}