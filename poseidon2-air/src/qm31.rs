use core::fmt::{Debug, Display};
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use num_bigint::BigUint;
use p3_field::extension::{BinomialExtensionField, Complex};
use p3_field::{ExtensionField, Field, FieldAlgebra, FieldExtensionAlgebra, Packable};
use p3_mersenne_31::{GenericPoseidon2LinearLayersMersenne31, Mersenne31, Poseidon2Mersenne31};
use serde::{Deserialize, Deserializer, Serialize};

type InnerQM31 = BinomialExtensionField<Complex<Mersenne31>, 2>;
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QM31(BinomialExtensionField<Complex<Mersenne31>, 2>);

// impl ExtensionField<Mersenne31> for QM31{
//     type ExtensionPacking = Self;
// }

impl Display for QM31 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl MulAssign<Mersenne31> for QM31 {
    fn mul_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Mul<Mersenne31> for QM31 {
    type Output = QM31;

    fn mul(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Mersenne31> for QM31 {
    fn sub_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Sub<Mersenne31> for QM31 {
    type Output = QM31;

    fn sub(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl Sub for QM31 {
    type Output = QM31;

    fn sub(self, rhs: QM31) -> Self::Output {
        todo!()
    }
}

impl SubAssign for QM31 {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl AddAssign<Mersenne31> for QM31 {
    fn add_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Add<Mersenne31> for QM31 {
    type Output = QM31;

    fn add(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl Add for QM31 {
    type Output = QM31;

    fn add(self, rhs: QM31) -> Self::Output {
        todo!()
    }
}

impl AddAssign for QM31 {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl From<Mersenne31> for QM31 {
    fn from(value: Mersenne31) -> Self {
        let cm31: Complex<Mersenne31> = value.into();
        Self(cm31.into())
    }
}

impl Neg for QM31 {
    type Output = QM31;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Mul for QM31 {
    type Output = QM31;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for QM31 {
    fn mul_assign(&mut self, rhs: Self) {
        MulAssign::mul_assign(&mut self.0, rhs.0)
    }
}

impl Product for QM31 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y).unwrap_or(Self::ONE)
    }
}

impl Sum for QM31 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or(Self::ZERO)
    }
}

impl Div for QM31 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Hash for QM31 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.0, state);
    }
}

impl Packable for QM31 {}

impl FieldAlgebra for QM31 {
    type F = QM31;

    const ZERO: Self = Self(InnerQM31::ZERO);

    const ONE: Self = Self(InnerQM31::ONE);

    const TWO: Self = Self(InnerQM31::TWO);

    const NEG_ONE: Self = Self(InnerQM31::NEG_ONE);

    fn from_f(f: Self::F) -> Self {
        Self(InnerQM31::from_f(f.0))
    }

    fn from_bool(b: bool) -> Self {
        Self(InnerQM31::from_bool(b))
    }

    fn from_canonical_u8(n: u8) -> Self {
        Self(InnerQM31::from_canonical_u8(n))
    }

    fn from_canonical_u16(n: u16) -> Self {
        Self(InnerQM31::from_canonical_u16(n))
    }

    fn from_canonical_u32(n: u32) -> Self {
        Self(InnerQM31::from_canonical_u32(n))
    }

    fn from_canonical_u64(n: u64) -> Self {
        Self(InnerQM31::from_canonical_u64(n))
    }

    fn from_canonical_usize(n: usize) -> Self {
        Self(InnerQM31::from_canonical_usize(n))
    }

    fn from_wrapped_u32(n: u32) -> Self {
        Self(InnerQM31::from_wrapped_u32(n))
    }

    fn from_wrapped_u64(n: u64) -> Self {
        Self(InnerQM31::from_wrapped_u64(n))
    }
}

impl FieldExtensionAlgebra<Mersenne31> for QM31 {
    const D: usize = 4;

    fn from_base(b: Mersenne31) -> Self {
        Self(InnerQM31::from_base(Complex::<Mersenne31>::from_base(b)))
    }

    fn from_base_slice(bs: &[Mersenne31]) -> Self {
        todo!()
    }

    fn from_base_fn<F: FnMut(usize) -> Mersenne31>(f: F) -> Self {
        todo!()
    }

    fn from_base_iter<I: Iterator<Item = Mersenne31>>(iter: I) -> Self {
        todo!()
    }

    fn as_base_slice(&self) -> &[Mersenne31] {
        todo!()
    }
}

impl Field for QM31 {
    type Packing = Self;

    const GENERATOR: Self = todo!();

    fn try_inverse(&self) -> Option<Self> {
        todo!()
    }

    fn order() -> BigUint {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct QM31ExtensionPacking(<BinomialExtensionField<Complex<Mersenne31>, 2> as ExtensionField<Complex<Mersenne31>>>::ExtensionPacking);

impl MulAssign<Mersenne31> for QM31ExtensionPacking {
    fn mul_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Mul<Mersenne31> for QM31ExtensionPacking {
    type Output = QM31ExtensionPacking;

    fn mul(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Mersenne31> for QM31ExtensionPacking {
    fn sub_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Sub<Mersenne31> for QM31ExtensionPacking {
    type Output = QM31ExtensionPacking;

    fn sub(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Mersenne31> for QM31ExtensionPacking {
    fn add_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Add<Mersenne31> for QM31ExtensionPacking {
    type Output = QM31ExtensionPacking;

    fn add(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl From<Mersenne31> for QM31ExtensionPacking {
    fn from(value: Mersenne31) -> Self {
        todo!()
    }
}

impl Product for QM31ExtensionPacking {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        todo!()
    }
}

impl Sum for QM31ExtensionPacking {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        todo!()
    }
}

impl Add for QM31ExtensionPacking {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for QM31ExtensionPacking {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Mul for QM31ExtensionPacking {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for QM31ExtensionPacking {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl SubAssign for QM31ExtensionPacking {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Sub for QM31ExtensionPacking {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Neg for QM31ExtensionPacking {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl FieldAlgebra for QM31ExtensionPacking {
    type F = QM31;

    const ZERO: Self = todo!();

    const ONE: Self = todo!();

    const TWO: Self = todo!();

    const NEG_ONE: Self = todo!();

    fn from_f(f: Self::F) -> Self {
        todo!()
    }

    fn from_bool(b: bool) -> Self {
        todo!()
    }

    fn from_canonical_u8(n: u8) -> Self {
        todo!()
    }

    fn from_canonical_u16(n: u16) -> Self {
        todo!()
    }

    fn from_canonical_u32(n: u32) -> Self {
        todo!()
    }

    fn from_canonical_u64(n: u64) -> Self {
        todo!()
    }

    fn from_canonical_usize(n: usize) -> Self {
        todo!()
    }

    fn from_wrapped_u32(n: u32) -> Self {
        todo!()
    }

    fn from_wrapped_u64(n: u64) -> Self {
        todo!()
    }
}

impl FieldExtensionAlgebra<Mersenne31> for QM31ExtensionPacking {
    const D: usize = todo!();

    fn from_base(b: Mersenne31) -> Self {
        todo!()
    }

    fn from_base_slice(bs: &[Mersenne31]) -> Self {
        todo!()
    }

    fn from_base_fn<F: FnMut(usize) -> Mersenne31>(f: F) -> Self {
        todo!()
    }

    fn from_base_iter<I: Iterator<Item = Mersenne31>>(iter: I) -> Self {
        todo!()
    }

    fn as_base_slice(&self) -> &[Mersenne31] {
        todo!()
    }
}

impl ExtensionField<Mersenne31> for QM31 {
    type ExtensionPacking = QM31ExtensionPacking;
}
