
![](https://raw.githubusercontent.com/At0micBee/scilib/master/branding/Scilib.png)

![](https://img.shields.io/docsrs/scilib?label=Tests&style=flat-square)
![](https://img.shields.io/crates/v/scilib?style=flat-square)
![](https://img.shields.io/crates/l/scilib?style=flat-square)

---

# Overview

This crate is designed to help any mathematical or scientific processes for the Rust community. It compiles many useful concepts and items that are key in many scientific domains. The aim of the crate is to provide these functions in pure Rust, to avoid any dependencies.

---

## Work in progress; What's coming?

As of the creation of this readme, I am working on this project mostly alone which means a few things:

1. The progression will be linked to my schedule
2. I will work firsts on concept I am familiar with
3. I am a self-taught developer, some solutions could be sub-optimal and thus improved, any help is welcome!

The schedule of the development of the crate is not clear, as I am for now writing this as a side project. I plan on adding many useful functions from a physics point of view, but will expand as I go. For now, my long term objectives are: Astrophysics, Thermodynamics, Quantum mechanics, Electromagnetism.

And hopefully more when this is done (statistics, integration tool, calculus, ...).

---

# Contents

## Useful mathematical functions

The Rust library doesn't provide some functions that are quite common in scientific processes, and this crate attempts to provide as many as it can. Euler's Gamma and Beta function, Newton's binomial, factorial, the error functions (erf, erfc, erfi), ...

```rust
// These functions can be found in the math crate
use scilib::math::basic::*;

let g = gamma(3.2);
let b = beta(-1.2, 2.5);

// The erf function can compute Complex numbers (erfc, erfi as well)
let c = Complex::from(-0.1, 0.7);
let e = erf(c);
```

---

## Coordinate systems

This crate provides functionalities for coordinate systems, such as Cartesian and Spherical, with many standard operations and conversions.

```rust
// They are found in the coordinate crate
use scilib::coordinate::*;

let car = cartesian::Cartesian::from(2.0, 1, 0.25);
let sph = spherical::Spherical::from_degree(1.2, 30, 60.2);
let cyl = spherical::Cylindrical::from_degree(1.2, 30, -2.55);
```

---

## Complex numbers

This crate provides basic functionalities for complex numbers, mainly to support its other goals. The implementation uses `f64` for both the real and imaginary parts, to ensure precision in the computations.

Basic operations have been implemented to facilitate their use, and should be pretty easy to manipulate.

```rust
// They are found in the complex crate
use scilib::math::complex::Complex;

let c1 = Complex::from(2, 3.5);
let c2 = Complex::from(-1.2, 4) * 2;
println!("{}", c1 + c2);
```

***More functionalities are on their way, they will be added as they are needed for other domains.***

---

## Bessel functions

Essential in many maths and physics domain, bessel function are solutions of Bessel's differential equation ([Wiki page](https://en.wikipedia.org/wiki/Bessel_function)). This crate provides functions for both real and complex numbers, and for integer or real function order. It covers standard Bessel functions, the spherical Bessel functions, and the Riccati-Bessel functions.

All functions are implemented:
- **J**: First kind
- **Y**: Second Kind
- **I**: Modified first kind
- **K**: Modified second kind
- **H1**: Hankel first kind
- **H2**: Hankel second kind
- **j**: Spherical first kind
- **y**: Spherical second kind
- **h1**: Spherical hankel first kind
- **h2**: Spherical hankel second kind
- **S**: Riccati-Bessel first kind
- **C**: Riccati-Bessel Second kind
- **Xi**: Riccati-Bessel Third kind
- **Zeta**: Riccati-Bessel Fourth kind

```rust
// Found in the math crate
use scilib::math::bessel;

// All functions support complex numbers, and real orders
let res_j = bessel::jf(-1.2, -2.3);             // J function; works for any input and order
let res_y = bessel::y(3.5, 1);                  // Y function; computes the limit for integer order
let res_i = bessel::i(7.2, 2.25);               // I function; similar to J
let res_k = bessel::k(-1.1, 0.5);               // K function; computes the limit for integer order
let res_1 = bessel::hankel_first(2, -2);        // Hankel first kind
let res_2 = bessel::hankel_second(1, -1.32);    // Hankel first kind
let res_sj = bessel::sj(4.4, 2);                // Spherical first kind
let res_sy = bessel::sy(-1.54, 3);              // Spherical second kind
let res_s1 = bessel::sh_first(2.11, 4);         // Spherical hankel first kind
let res_s2 = bessel::sh_second(0.253, 0);       // Spherical hankel second kind
```

Values are compared to known results (thanks, [WolframAlpha](https://www.wolframalpha.com/)), and the results are within small margins of error.

Thanks to [Neven](https://github.com/Gentil-N) for adding the Spherical versions.

---

## Signal functions

Support to conduct both fast Fourier transform (`fft`) and the inverse fast Fourier transform (`ifft`) is available. Computations are
done using [Bluestein's algorithm](https://en.wikipedia.org/wiki/Chirp_Z-transform#Bluestein.27s_algorithm). Convolution is also possible,
with any two vector sizes.

```rust
// Found in the fourier crate
use scilib::signal::*

// Computing values of the sinus
let r = range::linear(0.0, 10.0, 15);
let s: Vec<Complex> = r.iter().map(|val| val.sin()).collect();

let res = fft(&s);
let res2 = ifft(&res);
let res3 = convolve(&r, &s);
```

---

## Typical polynomials

Useful polynomials will be implemented to facilitate their uses to everyone; currently the [Legendre](https://docs.rs/scilib/0.4.0/scilib/math/polynomial/struct.Legendre.html), [Laguerre](https://docs.rs/scilib/0.4.0/scilib/math/polynomial/struct.Laguerre.html), [Bernoulli](https://docs.rs/scilib/0.4.0/scilib/math/polynomial/struct.Bernoulli.html) and [Euler](https://docs.rs/scilib/0.4.0/scilib/math/polynomial/struct.Euler.html) polynomials have been implemented.

```rust
// They are found in the polynomial crate
use scilib::math::polynomial::*;

// Legendre and Laguerre have their generalized versions
let leg = Legendre::new(2, -1); // l=2, m=-1
let lag = Laguerre::new(3, 2);  // l=3, m=2

// Standard support for Bernoulli and Euler (numbers and polynomials)
let ber = Bernoulli::new(3);    // n=3
let eul = Euler::new(5);        // n=5
```

---

## Quantum mechanics

Both the radial wave function Rnl(r) and the spherical harmonics Ylm(theta, phi) have been added to the quantum section. The Ylm is also valid for acoustics as well.

```rust
// Found in the quantum crate
use scilib::quantum::*;

// Computing Ylm for l=3, m=1, theta = 0.2 and phi = -0.3
let sph = spherical_harmonics(3, 1, 0.2, -0.3);

// Computing the Rnl for n=4, l=2
let rad = radial_wavefunction(4, 2, 1.3e-12);
```

---

## Constants

Many useful constants have been added, comprising many different fields, from astrophysics to quantum mechanics, but also mathematics, thermodynamics, electromagnetism, etc... They're listed in the `constant` module.

```rust
use scilib::constant;

println!("{}", constant::SUN_RADIUS);   // Solar radius
println!("{}", constant::H_BAR);        // H bar
println!("{}", constant::K_B);          // Boltzmann constant
println!("{}", constant::BOHR_MAG);     // Bohr magneton
// And many more...
```

---
