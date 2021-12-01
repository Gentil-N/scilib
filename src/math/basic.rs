//!
//! # Basic math functions
//! 
//! This module provides access to many useful function that are not provided by the base Rust.
//! 

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::f64::consts::{     // Using std lib constants
    FRAC_PI_2,              // Pi / 2
    FRAC_2_SQRT_PI,         // 2 / sqrt(Pi)
    TAU                     // Tau constant
};

use super::{                // Using parts from the crate
    super::constant,        // Calling scilib constants
    complex::Complex,       // Using Complex numbers
    polynomial::Bernoulli   // Bernoulli polynomials
};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Precision used for convergence
const PRECISION: f64 = 1.0e-12;

/// Stieltjes gamma computation precision
const STIELTJES_M: usize = 1_000_000;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// # Sinus cardinal
///
/// `x`: the value at which to evaluate the function
/// 
/// Returns: the sinc value of x
/// 
/// ```
/// # use scilib::math::basic::sinc;
/// let x: f64 = sinc(0.0);
/// let y: f64 = sinc(1.2);
/// 
/// // Comparing to tabulated values
/// assert_eq!(x, 1.0);
/// assert!((y - 0.776699238306) < 1.0e-12);
/// ```
pub fn sinc(x: f64) -> f64 {
    let res = x.sin() / x;
    if res.is_nan() {
        1.0
    } else {
        res
    }
}

/// # Newton binomial formula
///
/// `n` is the number of "objects" and `k` is the selection
/// 
/// Returns: `k` among `n`
/// 
/// ```
/// # use scilib::math::basic::binomial;
/// let res: usize = binomial(4_usize, 2_usize);
/// assert_eq!(res, 6);
/// ```
/// 
/// The implementation used here relies on the recurrence relation:
/// 
/// (n, k) = (n-1, k) + (n-1, k-1)
/// 
/// This pushes overflow back for a few more terms.
pub fn binomial(n: usize, k: usize) -> usize {

    // n must be greater than k to produce a value
    if k > n {
        return 0;
    }

    let mut res: usize = 1;             // We initialize the result at 1
    let mut n_d: usize = n;             // We need a mutable value for n

    // We loop the counter up to k possible values
    for val in 1..=k {
        res *= n_d;     // Changing the result by n
        n_d -= 1;       // Decrementing n
        res /= val;     // Changing the result by the choices options
    }

    // Returning the result
    res
}

/// # Factorial function
///
/// `n` is the integer at which to evaluate the factorial
/// 
/// Returns: `n`!
/// 
/// ```
/// # use scilib::math::basic::factorial;
/// let res: usize = factorial(5_usize);
/// assert_eq!(res, 120);
/// ```
pub fn factorial<T>(n: T) -> usize
where T: Into<usize> {
    (1..=n.into()).fold(1, |res, val| res * val)
}

/// # Stieltjes Gamma function
/// 
/// Computes Gamma_n(a).
pub fn stieltjes(n: usize, a: f64) -> f64 {

    let mut res: f64 = - (STIELTJES_M as f64 + a).ln().powi(n as i32 + 1) / (n as f64 + 1.0);

    for k in 0..STIELTJES_M {
        res += (k as f64 + a).ln().powi(n as i32) / (k as f64 + a);
    }

    res
}

/// # Hurwitz Zeta function
#[allow(dead_code)]
pub fn zeta<T, U>(s: T, a: U) -> f64
where T: Into<f64>, U: Into<f64> {

    // Conversions
    let mut a_c: f64 = a.into();
    let s_f: f64 = s.into();

    // If a is negative and even, we use Bernoulli
    if s_f == 0.0 || (s_f.is_sign_negative() && s_f % 2.0 == 0.0) {
        let ber: Bernoulli = Bernoulli::new(-s_f as usize + 1);
        return -ber.compute(a_c) / (-s_f + 1.0);
    }

    let mut res: f64 = 1.0 / (s_f - 1.0);

    let mut n: usize = 0;
    let mut sign: f64 = -1.0;
    let mut div: f64;
    let mut term: f64;

    'convergence: loop {

        if n >= 15 {
            break 'convergence;
        }

        sign *= -1.0;
        div = factorial(n) as f64;
        term = stieltjes(n, a_c);

        res += sign * term * (s_f - 1.0).powi(n as i32) / div;

        n += 1;
    }

    res

    /* // Sum values
    let mut n: Complex = Complex::unity();
    let mut term: Complex = Complex::unity() / a_c.powf(s_f);
    let mut res: Complex = term;

    let mut counter: usize = 0;

    'convergence: loop {

        if term.modulus() < PRECISION || counter >= 100000 {
            break 'convergence;
        }
        term = Complex::unity() / (a_c + n).powf(s_f);
        res += term;
        n += 1;
        counter += 1;
    }

    res */

    /* a_c.re = a_c.re.fract();

    let mut res: Complex = Complex::new();
    let mut n: usize = 0;

    let mut sign: f64;
    let mut term: Complex;
    let mut binom: f64;

    'convergence: loop {

        if n >= 60 {
            break 'convergence;
        }

        sign = -1.0;
        term = Complex::new();

        for k in 0..=n {
            sign *= -1.0;
            binom = binomial(n, k) as f64;
            term += sign * binom * (a_c + k as f64).powf(1.0 - s_f);
        }

        res += term / (n + 1) as f64;
        n += 1;
    }

    res / (s_f - 1.0) */

    /* // For the case Re{s} < 0 and 0 < a <=1

    let pre: f64 = 2.0 * gamma(1.0 - s_f) / (TAU).powf(1.0 - s_f);
    let w_s: f64 = (FRAC_PI_2 * s_f).sin();
    let w_c: f64 = (FRAC_PI_2 * s_f).cos();

    let mut term_c: Complex = Complex::new();
    let mut term_s: Complex = Complex::new();
    let mut val: Complex;
    let mut divisor: f64;

    let mut n: usize = 1;

    'convergence: loop {

        if n >= 10 {
            break 'convergence;
        }

        val = TAU * n as f64 * a_c;             // Pre-computing the value
        divisor = (n as f64).powf(1.0 - s_f);   // Pre-computing the divisor
        term_c += val.cos() / divisor;
        term_s += val.sin() / divisor;

        println!("{}", pre);

        n += 1;
    }

    pre * (w_s * term_c + w_c * term_s) */

}

/// # Gamma function
/// 
/// `x` is the value to evaluate.
/// 
/// The gamma function is the generalization of the factorial function, and is
/// used to provide value for non-integer numbers (except non-positive integers).
/// 
/// ```
/// # use scilib::math::basic::gamma;
/// let res_1: f64 = gamma(2.3);
/// let res_2: f64 = gamma(-0.45);
/// 
/// assert!((res_1 - 1.16671190).abs() < 1.0e-5);
/// assert!((res_2 - -3.591387).abs() < 1.0e-5);
/// ```
/// 
/// With the current computation scheme, we limit the precision of the computation in exchange for speed.
/// Typical values are achieve within a `1.0e-5` margin of error. Changing the method to another one
/// might grant some more speed and lower the error on the results.
pub fn gamma<T: Into<f64>>(value: T) -> f64 {

    let x: f64 = value.into();

    // If the number is an integer, we can simple return the factorial
    if x.fract() == 0.0 {
        return factorial(x as usize) as f64;
    }

    let mut n: f64 = 1.0;      // Order counter

    // The values of each term and the result
    let mut term: f64 = x.exp() / (1.0 + x);
    let mut res: f64 = 1.0;

    // If the first term is already too small we exit directly
    if (term - 1.0).abs() < PRECISION {
        return res;
    }

    // Computing the terms of the infinite series
    'convergence: loop {
        res *= term;

        //If the changes become too small, we stop
        if (term - 1.0).abs() < PRECISION {
            break 'convergence;
        }

        // Updating the values
        n += 1.0;
        term = (x / n).exp() / (1.0 + x / n);
    }

    res * (-x * constant::EULER_MASCHERONI).exp() / x
}

/// # Euler Beta function
/// 
/// The computation of the result is based on results from the gamma function. It is also possible to define
/// it with infinite series (or products), which could have some advantages, I'll investigate this possibility later.
/// 
/// ```
/// # use scilib::math::basic::beta;
/// let res: f64 = beta(1, 1.1);
/// 
/// assert!((res - 0.909090).abs() < 1.0e-5);
/// ```
pub fn beta<T, U>(x: T, y: U) -> f64
where T: Into<f64> + Copy, U: Into<f64> + Copy {

    let t1: f64 = gamma(x);
    let t2: f64 = gamma(y);
    let b: f64 = gamma(x.into() + y.into());
    
    t1 * t2 / b
}

/// # Error function
/// 
/// We define the error function for complex number, as its counterpart erfi requires.
/// 
/// ```
/// # use scilib::math::complex::Complex;
/// # use scilib::math::basic::erf;
/// let r = erf(2.1);
/// let c = erf(Complex::from(-0.1, 0.7));
/// 
/// assert!((r.re - 0.997021).abs() < 1.0e-5);
/// assert!((c.re - -0.18297754).abs() < 1.0e-5 && (c.im - 0.92747498).abs() < 1.0e-5);
/// ```
pub fn erf<T>(val: T) -> Complex
where T: Into<Complex> {

    let x: Complex = val.into();

    let mut n: f64 = 0.0;               // Index of iteration
    let mut d1: f64 = 1.0;              // First div
    let mut d2: f64;                    // Second div
    let mut sg: f64 = 1.0;              // Sign of the term
    
    let mut term: Complex = x;          // Term at each iter
    let mut res: Complex = 0.0.into();  // Result

    // If the term is too small we exit
    if term.modulus().abs() < PRECISION {
        return res;
    }

    'convergence: loop {
        res += term;

        // We exit when convergence reaches the precision
        if (term / res).modulus().abs() < PRECISION {
            break 'convergence;
        }

        n += 1.0;
        sg *= -1.0;
        d1 *= n;
        d2 = 2.0 * n + 1.0;
        term = sg * x.powf(d2) / (d1 * d2);
    }

    FRAC_2_SQRT_PI * res
}

/// # Complementary error function
/// 
/// This function simply returns the complement of the error function, that is `1 - erf(z)`.
/// 
/// ```
/// # use scilib::math::complex::Complex;
/// # use scilib::math::basic::erfc;
/// let c = Complex::from(1.25, 0.3);
/// let res = erfc(c);
/// 
/// assert!((res.re - 0.0505570).abs() < 1.0e-5 && (res.im - -0.0663174).abs() < 1.0e-5);
/// ```
pub fn erfc<T>(val: T) -> Complex
where T: Into<Complex> {
    Complex::unity() - erf(val)
}

/// # Imaginary error function
/// 
/// This function is defined as `-i * erf(i*z)`, which we take literally.
/// 
/// ```
/// # use scilib::math::complex::Complex;
/// # use scilib::math::basic::erfi;
/// let c = Complex::from(0.07, -1.1);
/// let res = erfi(c);
/// 
/// assert!((res.re - 0.02349883).abs() < 1.0e-5 && (res.im - -0.88201955).abs() < 1.0e-5);
/// ```
pub fn erfi<T>(val: T) -> Complex
where T: Into<Complex> {
    -Complex::i() * erf(Complex::i() * val)
}

/// # Builds the demanded Pascal's triangle line
/// 
/// The first position of the triangle is number as line 0, the rest follows from there.
/// 
/// ```
/// # use scilib::math::basic::pascal_triangle;
/// let l5: Vec<usize> = pascal_triangle(5);
/// 
/// assert_eq!(l5, vec![1, 5, 10, 10, 5, 1]);
/// ```
pub fn pascal_triangle(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(n);

    for k in 0..=n {
        res.push(binomial(n, k));
    }

    res
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
