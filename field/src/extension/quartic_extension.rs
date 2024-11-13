use alloc::{format, vec};
use alloc::string::ToString;
use alloc::vec::Vec;
use core::array;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use itertools::Itertools;
use num_bigint::BigUint;
use p3_util::convert_vec;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

use super::{
    BinomialExtensionField, Complex, ComplexExtendable, HasComplexBinomialExtension, HasFrobenius, HasTwoAdicBionmialExtension
};
use crate::extension::BinomiallyExtendable;
use crate::field::Field;
use crate::{
    field_to_array, AbstractExtensionField, AbstractField, ExtensionField, Packable, TwoAdicField,
};

type QuarticExtension<AF: AbstractField> = BinomialExtensionField<BinomialExtensionField<AF, 2>, 2>;

impl<AF: AbstractField> MulAssign<AF> for QuarticExtension<AF> {
    fn mul_assign(&mut self, _: AF) {
        todo!()
    }
}

impl<AF> Mul<AF> for QuarticExtension<AF>
where
    AF: AbstractField + BinomiallyExtendable<2>,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: AF) -> QuarticExtension<AF> {
        let [a, b] = self.value.clone();
        let real: BinomialExtensionField<AF, 2> = Mul::<AF>::mul(a, rhs);
        let imaginary: BinomialExtensionField<AF, 2> = Mul::<AF>::mul(b, rhs);
        QuarticExtension::<AF> {
            value: [real, imaginary],
        }
    }
}

impl<AF> Sub<AF> for QuarticExtension<AF>
where
    AF: AbstractField + BinomiallyExtendable<2>,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    type Output = Self;

    fn sub(self, rhs: AF) -> Self::Output {
        let [mut extension_real_part, extension_imaginary_part] = self.value.clone();
        extension_real_part.value[0] = extension_real_part.value[0] - rhs;
        Self {
            value: [extension_real_part, extension_imaginary_part],
        }
    }
}

impl<AF> SubAssign<AF> for QuarticExtension<AF>
where
    AF: AbstractField + BinomiallyExtendable<2>,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: AF) {
        *self = self.clone() - rhs;
    }
}

impl<AF> Add<AF> for QuarticExtension<AF>
where
    AF: AbstractField + BinomiallyExtendable<2>,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    type Output = Self;

    fn add(self, rhs: AF) -> Self::Output {
        let [mut extension_real_part, extension_imaginary_part] = self.value.clone();
        extension_real_part.value[0] = extension_real_part.value[0] + rhs;
        Self {
            value: [extension_real_part, extension_imaginary_part],
        }
    }
}

impl<AF> AddAssign<AF> for QuarticExtension<AF>
where
    AF: AbstractField + BinomiallyExtendable<2>,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    #[inline]
    fn add_assign(&mut self, rhs: AF) {
        *self = self.clone() + rhs;
    }
}

impl<AF> From<AF> for QuarticExtension<AF> 
where 
    AF: AbstractField,
    AF::F: BinomiallyExtendable<2>,
    BinomialExtensionField<AF, 2>: AbstractField + BinomiallyExtendable<2>,
{
    fn from(x: AF) -> Self {
        let mut res = Self::ZERO;
        let [mut extension_real_part, extension_imaginary_part] = res.value.clone();
        extension_real_part.value[0] = x;
        Self {
            value: [extension_real_part, extension_imaginary_part]
        }
    }
}



impl<AF> AbstractExtensionField<AF> for QuarticExtension<AF>
where
    AF: AbstractField + ComplexExtendable,
    AF::F: BinomiallyExtendable<2> + HasComplexBinomialExtension<2>,
    AF: Copy,
    BinomialExtensionField<AF, 2>: BinomiallyExtendable<2> + AbstractExtensionField<AF>,
{
    const D: usize = 2;

    #[inline]
    fn from_base(b: AF) -> Self {
            Self::from(b)
    }

    #[inline]
    fn from_base_slice(bs: &[AF]) -> Self {
        assert!(bs.len() == 4);
        let [mut real, mut imaginary] = Self::ZERO.value;
        real.value[0] = bs[0];
        real.value[1] = bs[1];
        imaginary.value[0] = bs[2];
        imaginary.value[1] = bs[3];
        Self{ value: [real, imaginary]}
    }

    #[inline]
    fn from_base_fn<F: FnMut(usize) -> AF>(f: F) -> Self {
        let base_field_array: [AF; 4] = array::from_fn(f);
        
        Self::from_base_slice(&base_field_array)
    }

    #[inline]
    fn from_base_iter<I: Iterator<Item = AF>>(iter: I) -> Self {
        let mut res = Self::default();
        let mut element_array = vec![];
        for (i, b) in iter.enumerate() {
            element_array.push(b);
        }
        Self::from_base_slice(&element_array)
    }

    #[inline]
    fn as_base_slice(&self) -> &[AF] {
        // let [real, imaginary] = self.value;
        // let res: &[AF] = vec![real.value[0], real.value[1], imaginary.value[0], imaginary.value[1]].as_slice();
        // // res
        // &[self.value[0].value[0],self.value[0].value[1], self.value[1].value[0], self.value[1].value[1]]
        &self.value[0].value
    }

}

// */
