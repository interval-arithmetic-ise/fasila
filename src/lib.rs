#![no_std]

// #![feature(riscv_ext_intrinsics)]

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Interval32(f64);

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy)]
struct Bounds32 {
    lower: f32,
    upper: f32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy)]
struct Bounds32 {
    upper: f32,
    lower: f32,
}

impl Bounds32 {
    fn new(lower: f32, upper: f32) -> Self {
        assert!(lower <= upper);
        Bounds32 { lower, upper }
    }
}

impl From<Bounds32> for Interval32 {
    fn from(repr: Bounds32) -> Interval32 {
        // SAFETY: TODO
        unsafe { core::mem::transmute(repr) }
    }
}

impl From<Interval32> for Bounds32 {
    fn from(interval: Interval32) -> Bounds32 {
        // SAFETY: TODO
        unsafe { core::mem::transmute(interval) }
    }
}

impl Bounds32 {
    fn interval(&self) -> Interval32 {
        (*self).into()
    }
}

impl Interval32 {
    pub fn new(lower: f32, upper: f32) -> Self {
        Bounds32::new(lower, upper).into()
    }

    fn bounds(&self) -> Bounds32 {
        (*self).into()
    }

    #[inline]
    pub fn add(self, rhs: Self) -> Self {
        let a = self.bounds();
        let b = rhs.bounds();
        Bounds32::new(
            a.lower + b.lower,
            a.upper + b.upper
        ).into()
    }

    #[inline]
    pub fn sub(self, rhs: Self) -> Self {
        let a = self.bounds();
        let b = rhs.bounds();
        Bounds32::new(
            a.lower - b.upper,
            a.upper - b.lower,
        )
        .into()
    }

    #[inline]
    pub fn mul(self, rhs: Self) -> Self {
        let a = self.bounds();
        let b = rhs.bounds();
        let p1 = a.lower * b.lower;
        let p2 = a.lower * b.upper;
        let p3 = a.upper * b.lower;
        let p4 = a.upper * b.upper;
        let lower = p1.min(p2).min(p3).min(p4);
        let upper = p1.max(p2).max(p3).max(p4);
        Bounds32::new(lower, upper).into()
    }

    // FIXME: The division behavior is not exact when the denominator contains
    // zero, currently it just panics, but do we really want this to fail?
    #[inline]
    pub fn div(self, rhs: Self) -> Self {
        let a = self.bounds();
        let b = rhs.bounds();

        assert!(!(b.lower <= 0.0 && b.upper >= 0.0));

        let q1 = a.lower / b.lower;
        let q2 = a.lower / b.upper;
        let q3 = a.upper / b.lower;
        let q4 = a.upper / b.upper;
        let lower = q1.min(q2).min(q3).min(q4);
        let upper = q1.max(q2).max(q3).max(q4);
        Bounds32::new(lower, upper).into()
    }

    #[inline(always)]
    pub fn mid(self) -> f32 {
        let b = self.bounds();
        (b.lower + b.upper) / 2.0
    }

    #[inline(always)]
    pub fn width(self) -> f32 {
        let b = self.bounds();
        b.upper - b.lower
    }

    #[inline(always)]
    pub fn radius(self) -> f32 {
        let b = self.bounds();
        (b.upper - b.lower) / 2.0
    }

    #[inline(always)]
    pub fn min(self) -> f32 {
        self.bounds().lower
    }

    #[inline(always)]
    pub fn max(self) -> f32 {
        self.bounds().upper
    }

    #[inline(always)]
    pub fn before(self, rhs: Self) -> bool {
        self.bounds().upper < rhs.bounds().lower
    }

    #[inline(always)]
    pub fn meets(self, rhs: Self) -> bool {
        self.bounds().upper == rhs.bounds().lower
    }

    #[inline]
    pub fn overlaps(self, rhs: Self) -> bool {
        let a = self.bounds();
        let b = rhs.bounds();
        (a.lower < b.lower && a.upper < b.upper && a.upper > b.lower) ||
        (b.lower < a.lower && b.upper < a.upper && b.upper > a.lower)
    }

    #[inline(always)]
    pub fn during(self, rhs: Self) -> bool {
        let a = self.bounds();
        let b = rhs.bounds();
        b.lower < a.lower && a.upper < b.upper
    }

    #[inline(always)]
    pub fn starts(self, rhs: Self) -> bool {
        let a = self.bounds();
        let b = rhs.bounds();
        a.lower == b.lower && a.upper < b.upper
    }

    #[inline(always)]
    pub fn finishes(self, rhs: Self) -> bool {
        let a = self.bounds();
        let b = rhs.bounds();
        a.upper == b.upper && a.lower > b.lower
    }

    #[inline(always)]
    pub fn eq(self, rhs: Self) -> bool {
        let a = self.bounds();
        let b = rhs.bounds();
        a.lower == b.lower && a.upper == b.upper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(3.0, 4.0);
        let result = i1.add(i2);
        let bounds = result.bounds();
        // Expected: [1+3, 2+4] = [4,6]
        assert_eq!(bounds.lower, 4.0);
        assert_eq!(bounds.upper, 6.0);
    }

    #[test]
    fn test_sub() {
        let i1 = Interval32::new(5.0, 7.0);
        let i2 = Interval32::new(2.0, 3.0);
        let result = i1.sub(i2);
        let bounds = result.bounds();
        // For subtraction, [a, b] - [c, d] = [a - d, b - c]
        // Expected: [5-3, 7-2] = [2,5]
        assert_eq!(bounds.lower, 2.0);
        assert_eq!(bounds.upper, 5.0);
    }

    #[test]
    fn test_mul() {
        // Test with positive intervals.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(3.0, 4.0);
        let result = i1.mul(i2);
        let bounds = result.bounds();
        // Products: 1*3=3, 1*4=4, 2*3=6, 2*4=8 => [3,8]
        assert_eq!(bounds.lower, 3.0);
        assert_eq!(bounds.upper, 8.0);

        // Test with negative interval.
        let i3 = Interval32::new(-2.0, -1.0);
        let i4 = Interval32::new(3.0, 4.0);
        let result2 = i3.mul(i4);
        let bounds2 = result2.bounds();
        // Products: (-2*3=-6, -2*4=-8, -1*3=-3, -1*4=-4) => min=-8, max=-3
        assert_eq!(bounds2.lower, -8.0);
        assert_eq!(bounds2.upper, -3.0);
    }

    #[test]
    #[should_panic]
    fn test_div_invalid() {
        // Test division by an interval containing zero should panic.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(-1.0, 1.0);
        let result2 = i1.div(i2);
    }

    #[test]
    fn test_div_valid() {
        // Test with divisor not containing zero.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(3.0, 4.0);
        let result = i1.div(i2);
        let bounds = result.bounds();
        // Divisions: 1/3≈0.333, 1/4=0.25, 2/3≈0.667, 2/4=0.5 => [0.25, ~0.667]
        assert!((bounds.lower - 0.25).abs() < 1e-6);
        assert!((bounds.upper - 0.6666667).abs() < 1e-6);
    }

    #[test]
    fn test_mid() {
        let i = Interval32::new(1.0, 3.0);
        // Midpoint: (1+3)/2 = 2.0
        assert_eq!(i.mid(), 2.0);
    }

    #[test]
    fn test_width() {
        let i = Interval32::new(1.0, 4.0);
        // Width: 4-1 = 3.0
        assert_eq!(i.width(), 3.0);
    }

    #[test]
    fn test_radius() {
        let i = Interval32::new(1.0, 5.0);
        // Radius: (5-1)/2 = 2.0
        assert_eq!(i.radius(), 2.0);
    }

    #[test]
    fn test_min() {
        let i = Interval32::new(2.0, 6.0);
        // Lower bound is 2.0
        assert_eq!(i.min(), 2.0);
    }

    #[test]
    fn test_max() {
        let i = Interval32::new(2.0, 6.0);
        // Upper bound is 6.0
        assert_eq!(i.max(), 6.0);
    }

    #[test]
    fn test_before() {
        // "Before": self.upper < other.lower.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(3.0, 4.0);
        assert!(i1.before(i2));
        assert!(!i2.before(i1));
    }

    #[test]
    fn test_meets() {
        // "Meets": self.upper == other.lower.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(2.0, 3.0);
        assert!(i1.meets(i2));
        assert!(!i2.meets(i1));
    }

    #[test]
    fn test_overlaps() {
        // "Overlaps": intervals partially overlap without one fully containing the other.
        let i1 = Interval32::new(1.0, 3.0);
        let i2 = Interval32::new(2.0, 4.0);
        assert!(i1.overlaps(i2));
        // Non-overlapping intervals.
        let i3 = Interval32::new(4.0, 5.0);
        assert!(!i1.overlaps(i3));
    }

    #[test]
    fn test_during() {
        // "During": self is entirely contained within other.
        let inner = Interval32::new(3.0, 4.0);
        let outer = Interval32::new(2.0, 5.0);
        assert!(inner.during(outer));
        assert!(!outer.during(inner));
    }

    #[test]
    fn test_starts() {
        // "Starts": same lower bound, and self.upper < other.upper.
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(1.0, 3.0);
        assert!(i1.starts(i2));
        assert!(!i2.starts(i1));
    }

    #[test]
    fn test_finishes() {
        // "Finishes": same upper bound, and self.lower > other.lower.
        let i1 = Interval32::new(2.0, 3.0);
        let i2 = Interval32::new(1.0, 3.0);
        assert!(i1.finishes(i2));
        assert!(!i2.finishes(i1));
    }

    #[test]
    fn test_eq() {
        let i1 = Interval32::new(1.0, 2.0);
        let i2 = Interval32::new(1.0, 2.0);
        let i3 = Interval32::new(2.0, 3.0);
        assert!(i1.eq(i2));
        assert!(!i1.eq(i3));
    }
}
