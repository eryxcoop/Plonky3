use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::array;
use core::fmt::{self, Debug, Display, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use itertools::Itertools;
use num_bigint::BigUint;
use p3_util::convert_vec;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

use super::{BinomialExtensionField, HasComplexBinomialExtension, HasFrobenius, HasTwoAdicBionmialExtension};
use crate::extension::BinomiallyExtendable;
use crate::field::Field;
use crate::{
    field_to_array, AbstractExtensionField, AbstractField, ExtensionField, Packable, TwoAdicField,
};


type QuarticExtension<AF: AbstractField> = BinomialExtensionField<BinomialExtensionField<AF, 2>, 2>;

impl <AF: AbstractField> MulAssign<AF> for QuarticExtension<AF>{
    fn mul_assign(&mut self, _: AF) { todo!() }
}

impl <AF: AbstractField> Mul<AF> for QuarticExtension<AF>{
    fn mul(&mut self, _: AF) { todo!() }
}

impl<AF> AbstractExtensionField<AF> for QuarticExtension<AF>
where
    AF: AbstractField,

    AF::F: BinomiallyExtendable<D1> + HasComplexBinomialExtension<D2>,
    AF: Copy,
    BinomialExtensionField<AF, D1>: BinomiallyExtendable<D2> + AbstractExtensionField<AF>,
{
    const D: usize = D2;

    #[inline]
    fn from_base(b: AF) -> Self {
        
            BinomialExtensionField{
                value: [BinomialExtensionField{value: [b; D1]}; D2]
            }
        
    }

    #[inline]
    fn from_base_slice(bs: &[AF]) -> Self {
        Self::from_base_fn(|i| bs[i].clone())
    }

    #[inline]
    fn from_base_fn<F: FnMut(usize) -> AF>(f: F) -> Self {
        Self {
            value: array::from_fn(f),
        }
    }

    #[inline]
    fn from_base_iter<I: Iterator<Item = AF>>(iter: I) -> Self {
        let mut res = Self::default();
        for (i, b) in iter.enumerate() {
            res.value[i] = b;
        }
        res
    }

    #[inline]
    fn as_base_slice(&self) -> &[AF] {
        &self.value
    }
}

// impl<
//     FB,
//     const D1: usize,
//     const D2: usize
// >
//     ExtensionField<FB>
//     for BinomialExtensionField<BinomialExtensionField<FB, D1>, D2>

// where
//     FB: BinomiallyExtendable<D1>,
//     BinomialExtensionField<FB, D1>: BinomiallyExtendable<D2>,
// {
//     type ExtensionPacking = BinomialExtensionField<F::Packing, D>;
// }




/* 
impl<F, F2, F3> ExtensionField<F>
    for F3
    where F: Field,
    F2: ExtensionField<F>,
    F3: ExtensionField<F2>
{
    type ExtensionPacking = BinomialExtensionField<F::Packing, D>;
}*/