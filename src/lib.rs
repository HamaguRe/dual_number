// Dual-Number libraly
// f32 & f64

pub type DualNumber<T> = (T, T);  // (real part, dual part)

use num_traits::cast;
use num_traits::float::Float;


#[inline(always)]
pub fn add<T>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T>
where T: Float {
    (a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
pub fn sub<T>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T>
where T: Float {
    (a.0 - b.0, a.1 - b.1)
}

#[inline(always)]
pub fn scale<T>(s: T, d: DualNumber<T>) -> DualNumber<T>
where T: Float {
    (s * d.0, s * d.1)
}

/// s*a + b
/// add( scale(s, a), b )
#[inline(always)]
pub fn scale_add<T>(s: T, a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T>
where T: Float {
    ( s.mul_add(a.0, b.0), s.mul_add(a.1, b.1) )
}

/// Calculate a*b (=b*a)
#[inline(always)]
pub fn mul<T>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T>
where T: Float {
    (a.0*b.0, a.0*b.1 + a.1*b.0)
}

/// a / b
/// b の実部が0の場合には成立しない
#[inline(always)]
pub fn div<T>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T>
where T: Float {
    let real = a.0 / b.0;
    let dual = (b.0 * a.1 - b.1 * a.0) / (b.0 * b.0);
    (real, dual)
}

#[inline(always)]
pub fn conj<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    (a.0, -a.1)
}

/// 実部が0の場合には成立しない
#[inline(always)]
pub fn inv<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    conj(( a.0.recip(), a.1 / (a.0 * a.0) ))
}

/// 実部が0の場合には成立しない
#[inline(always)]
pub fn sqrt<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    let half: T = cast(0.5).unwrap();
    let tmp = a.0.sqrt();
    ( tmp, half * (a.1 / tmp) )
}

#[inline(always)]
pub fn sin<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    let f = a.0.sin_cos();
    (f.0, a.1 * f.1)
}

#[inline(always)]
pub fn cos<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    let f = a.0.sin_cos();
    (f.1, -a.1 * f.0)
}

#[inline(always)]
pub fn exp<T>(a: DualNumber<T>) -> DualNumber<T>
where T: Float {
    let tmp = a.0.exp();
    (tmp, tmp * a.1)
}