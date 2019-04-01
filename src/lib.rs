// f32 or f64

extern crate num_traits;
use num_traits::float::Float;

pub type DualNumber<T> = (T, T);  // (real part, dual part)


#[inline(always)]
pub fn add<T: Float>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T> {
    (a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
pub fn mul<T: Float>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T> {
    (a.0*b.0, a.0*b.1 + a.1*b.0)
}

/// a / b
#[inline(always)]
pub fn div<T: Float>(a: DualNumber<T>, b: DualNumber<T>) -> DualNumber<T> {
    let real = a.0 / b.0;
    let dual = (b.0 * a.1 - b.1 * a.0) / (b.0 * b.0);
    (real, dual)
}

#[inline(always)]
pub fn conj<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    (a.0, -a.1)
}

#[inline(always)]
pub fn inverse<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    let one = T::one();

    let real = one / a.0;
    let dual = -a.1 / (a.0 * a.0);
    (real, dual)
}

#[inline(always)]
pub fn sqrt<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    let two = T::one() + T::one();

    let real = a.0.sqrt();
    let dual = a.1 / ( two * a.0.sqrt() );
    (real, dual)
}

#[inline(always)]
pub fn sin<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    let real = a.0.sin();
    let dual = a.1 * a.0.cos();
    (real, dual)
}

#[inline(always)]
pub fn cos<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    let real = a.0.cos();
    let dual = -a.1 * a.0.sin();
    (real, dual)
}

#[inline(always)]
pub fn exp<T: Float>(a: DualNumber<T>) -> DualNumber<T> {
    let tmp = a.0.exp();
    (tmp, tmp * a.1)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
