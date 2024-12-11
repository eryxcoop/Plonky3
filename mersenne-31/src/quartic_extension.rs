use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::array;
use core::fmt::{self, Debug, Display, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use extension::BinomiallyExtendable;
use itertools::Itertools;
use num_bigint::BigUint;
use p3_field::extension::{self, BinomialExtensionField};
use p3_field::FieldAlgebra;
use p3_util::convert_vec;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

use crate::Mersenne31;

type QuarticExtension = BinomialExtensionField<BinomialExtensionField<Mersenne31, 2>, 2>;

#[cfg(test)]
mod tests {
    use extension::Complex;
    use itertools::Itertools;
    use {FieldAlgebra, Mersenne31};

    use super::*;

    type CM31 = Complex<Mersenne31>;
    type QM31 = QuarticExtension;

    fn get_quartic_extension_element() -> (
        Mersenne31,
        Mersenne31,
        Mersenne31,
        Mersenne31,
        Complex<Mersenne31>,
        Complex<Mersenne31>,
        QuarticExtension,
    ) {
        let a = Mersenne31::new(1);
        let b = Mersenne31::new(2);
        let c = Mersenne31::new(3);
        let d = Mersenne31::new(4);
        let real: Complex<Mersenne31> = BinomialExtensionField::new(a, b);

        let imaginary: Complex<Mersenne31> = BinomialExtensionField::new(c, d);
        let quartic_element: QuarticExtension = QM31::new(real, imaginary);
        (a, b, c, d, real, imaginary, quartic_element)
    }

    #[test]
    fn test_can_multiply_with_base_field() {
        let factor: Mersenne31 = Mersenne31::new(2);
        let (a, b, c, d, real, imaginary, quartic_element) = get_quartic_extension_element();
        let expected_real = BinomialExtensionField::new(a * factor, b * factor);
        let expected_imaginary = BinomialExtensionField::new(c * factor, d * factor);
        let expected_result = QM31::new(expected_real, expected_imaginary);

        let multiplication = Mul::<Mersenne31>::mul(quartic_element, factor);

        assert_eq!(multiplication, expected_result);
    }

    #[test]
    fn test_can_subtract_with_base_field() {
        let factor: Mersenne31 = Mersenne31::new(1);
        let (a, b, c, d, real, imaginary, quartic_element) = get_quartic_extension_element();

        let expected_real = BinomialExtensionField::new(Mersenne31::ZERO, b);
        let expected_imaginary = BinomialExtensionField::new(c, d);

        let expected_result = QM31::new(expected_real, expected_imaginary);

        let subtraction = Sub::<Mersenne31>::sub(quartic_element, factor);
        assert_eq!(subtraction, expected_result);
    }

    #[test]
    fn test_can_subtract_assign_with_base_field() {
        let factor: Mersenne31 = Mersenne31::new(1);
        let (a, b, c, d, real, imaginary, mut quartic_element) = get_quartic_extension_element();

        let expected_real = BinomialExtensionField::new(Mersenne31::ZERO, b);
        let expected_imaginary = BinomialExtensionField::new(c, d);

        let expected_result = QM31::new(expected_real, expected_imaginary);

        SubAssign::<Mersenne31>::sub_assign(&mut quartic_element, factor);
        assert_eq!(quartic_element, expected_result);
    }

    #[test]
    fn test_can_add_assign_with_base_field() {
        let factor: Mersenne31 = Mersenne31::new(1);
        let (a, b, c, d, _, _, mut quartic_element) = get_quartic_extension_element();
        let expected_real = BinomialExtensionField::new(Mersenne31::TWO, b);
        let expected_imaginary = BinomialExtensionField::new(c, d);
        let expected_result = QM31::new(expected_real, expected_imaginary);

        AddAssign::<Mersenne31>::add_assign(&mut quartic_element, factor);

        assert_eq!(quartic_element, expected_result);
    }

    #[test]
    fn test_can_instantiate_from_base_field() {
        let element = QuarticExtension::from(Mersenne31::new(1));

        let a = Mersenne31::new(1);
        let b = Mersenne31::new(0);
        let c = Mersenne31::new(0);
        let d = Mersenne31::new(0);
        let real: Complex<Mersenne31> = BinomialExtensionField::new(a, b);

        let imaginary: Complex<Mersenne31> = BinomialExtensionField::new(c, d);
        let expected_result: QuarticExtension = QM31::new(real, imaginary);

        assert_eq!(element, expected_result);
    }
}
