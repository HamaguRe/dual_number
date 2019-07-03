// f64 only

pub type DualNumber<T> = (T, T);  // (real part, dual part)


#[inline(always)]
pub fn add(a: DualNumber<f64>, b: DualNumber<f64>) -> DualNumber<f64> {
    (a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
pub fn scale(s: f64, d: DualNumber<f64>) -> DualNumber<f64> {
    (s * d.0, s * d.1)
}

#[inline(always)]
pub fn mul(a: DualNumber<f64>, b: DualNumber<f64>) -> DualNumber<f64> {
    (a.0*b.0, a.0*b.1 + a.1*b.0)
}

/// a / b
#[inline(always)]
pub fn div(a: DualNumber<f64>, b: DualNumber<f64>) -> DualNumber<f64> {
    let real = a.0 / b.0;
    let dual = (b.0 * a.1 - b.1 * a.0) / (b.0 * b.0);
    (real, dual)
}

#[inline(always)]
pub fn conj(a: DualNumber<f64>) -> DualNumber<f64> {
    (a.0, -a.1)
}

#[inline(always)]
pub fn inverse(a: DualNumber<f64>) -> DualNumber<f64> {
    let tmp = (a.0 * a.0).recip();
    scale( tmp,  (a.0, -a.1) )
}

#[inline(always)]
pub fn sqrt(a: DualNumber<f64>) -> DualNumber<f64> {
    let real = a.0.sqrt();
    let dual = a.1 / ( 2.0 * a.0.sqrt() );
    (real, dual)
}

#[inline(always)]
pub fn sin(a: DualNumber<f64>) -> DualNumber<f64> {
    let f = a.0.sin_cos();
    (f.0, a.1 * f.1)
}

#[inline(always)]
pub fn cos(a: DualNumber<f64>) -> DualNumber<f64> {
    let f = a.0.sin_cos();
    (f.1, -a.1 * f.0)
}

#[inline(always)]
pub fn exp(a: DualNumber<f64>) -> DualNumber<f64> {
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
