pub type DualNumber = (f64, f64);  // (real part, dual part)


#[inline(always)]
pub fn add(a: DualNumber, b: DualNumber) -> DualNumber {
    (a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
pub fn mul(a: DualNumber, b: DualNumber) -> DualNumber {
    (a.0*b.0, a.0*b.1 + a.1*b.0)
}

/// "a" / "b"
#[inline(always)]
pub fn div(a: DualNumber, b: DualNumber) -> DualNumber {
    let real = a.0 / b.0;
    let dual = (b.0 * a.1 - b.1 * a.0) / (b.0 * b.0);
    (real, dual)
}

#[inline(always)]
pub fn conj(a: DualNumber) -> DualNumber {
    (a.0, -a.1)
}

#[inline(always)]
pub fn inverse(a: DualNumber) -> DualNumber {
    let real = 1.0 / a.0;
    let dual = -a.1 / (a.0 * a.0);
    (real, dual)
}

#[inline(always)]
pub fn sqrt(a: DualNumber) -> DualNumber {
    let real = a.0.sqrt();
    let dual = a.0 / ( 2.0 * a.1.sqrt() );
    (real, dual)
}

#[inline(always)]
pub fn sin(a: DualNumber) -> DualNumber {
    let real = a.0.sin();
    let dual = a.1 * a.0.cos();
    (real, dual)
}

#[inline(always)]
pub fn cos(a: DualNumber) -> DualNumber {
    let real = a.0.cos();
    let dual = -a.1 * a.0.sin();
    (real, dual)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
