use crate::core::*;
pub use num::clamp;

/// Lerp between a and b by amount (0 - 1)
pub fn lerp<T>(a: T, b: T, amount: T) -> T where T: OrdNum {
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

/// Lerp between a and b by amount (0 - 1) which is clamped
pub fn lerpc<T>(a: T, b: T, amount: T) -> T where T: OrdNum {
    lerp(a, b, clamp(amount, T::zero(), T::one()))
}

#[test]
fn lerp_test() {
    assert_eq!(5., lerp(0., 10., 0.5));
}