use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use p3_field::{ExtensionField, FieldAlgebra, FieldExtensionAlgebra};
use p3_mersenne_31::{GenericPoseidon2LinearLayersMersenne31, Mersenne31, Poseidon2Mersenne31};


#[derive(Debug)]
pub struct QM31;

impl MulAssign<Mersenne31> for QM31{
    fn mul_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Mul<Mersenne31> for QM31{
    type Output = QM31;

    fn mul(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Mersenne31> for QM31{
    fn sub_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Sub<Mersenne31> for QM31{
    type Output = QM31;

    fn sub(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl Sub for QM31{
    type Output = QM31;

    fn sub(self, rhs: QM31) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Mersenne31> for QM31{

    fn add_assign(&mut self, rhs: Mersenne31) {
        todo!()
    }
}

impl Add<Mersenne31> for QM31{
    type Output = QM31;

    fn add(self, rhs: Mersenne31) -> Self::Output {
        todo!()
    }
}

impl Add for QM31{
    type Output = QM31;

    fn add(self, rhs: QM31) -> Self::Output {
        todo!()
    }
}

impl From<Mersenne31> for QM31{
    fn from(value: Mersenne31) -> Self {
        todo!()
    }
}

impl FieldAlgebra for QM31{
    type F = Mersenne31;

    const ZERO: Self = todo!();

    const ONE: Self= todo!();

    const TWO: Self= todo!();

    const NEG_ONE: Self= todo!();

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

impl FieldExtensionAlgebra<Mersenne31> for QM31{
    const D: usize;

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

impl ExtensionField<Mersenne31> for QM31{
    type ExtensionPacking;
}