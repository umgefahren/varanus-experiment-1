use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::simd::PortableSimdElement;

pub trait VersionNumber: PartialOrd + PortableSimdElement + Shl + Shr + ShlAssign + ShrAssign + Copy + Sized {
    fn max() -> Self;
    fn min() -> Self;
}

impl VersionNumber for u8 {

   #[inline]
   fn max() -> Self {
       u8::MAX
   }

   #[inline]
   fn min() -> Self {
       u8::MAX
   }

}

impl VersionNumber for u16 {

    #[inline]
    fn max() -> Self {
        u16::MAX
    }

    #[inline]
    fn min() -> Self {
        u16::MIN
    }
}

impl VersionNumber for u32 {

    #[inline]
    fn max() -> Self {
        u32::MAX
    }

    #[inline]
    fn min() -> Self {
        u32::MIN
    }
}

impl VersionNumber for u64 {

    #[inline]
    fn max() -> Self {
        u64::MAX
    }

    #[inline]
    fn min() -> Self {
        u64::MIN
    }
}

impl VersionNumber for usize {

    #[inline]
    fn max() -> Self {
        usize::MAX
    }

    #[inline]
    fn min() -> Self {
        usize::MIN
    }
}

impl VersionNumber for i8 {

    #[inline]
    fn max() -> Self {
        i8::MAX
    }

    #[inline]
    fn min() -> Self {
        i8::MIN
    }
}

impl VersionNumber for i16 {

    #[inline]
    fn max() -> Self {
        i16::MAX
    }

    #[inline]
    fn min() -> Self {
        i16::MIN
    }
}

impl VersionNumber for i32 {

    #[inline]
    fn max() -> Self {
        i32::MAX
    }

    #[inline]
    fn min() -> Self {
        i32::MIN
    }
}

impl VersionNumber for i64 {

    #[inline]
    fn max() -> Self {
        i64::MAX
    }

    #[inline]
    fn min() -> Self {
        i64::MIN
    }
}

impl VersionNumber for isize {
    #[inline]
    fn max() -> Self {
        isize::MAX
    }

    #[inline]
    fn min() -> Self {
        isize::MIN
    }
}

#[inline]
pub fn serial_compare<N: VersionNumber>(major: N, minor: N, patch: N) -> bool {
    let max = N::max();
    let min = N::min();
    major != max && major != min && minor != max && minor != min && patch != max && patch != min
}

fn fast_compare<N: VersionNumber>(major: N, minor: N, patch: N) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(nightly)] {
            return crate::simd::fast_compare_simd(major, minor, patch);
        } else {
            return serial_compare(major,minor,patch);
        }
    }
}

#[derive(Debug)]
pub enum NewVersionError {
    MajorIsMax,
    MajorIsMin,
    MinorIsMax,
    MinorIsMin,
    PatchIsMax,
    PatchIsMin
}


#[derive(Debug)]
#[cfg_attr(nightly, repr(simd))]
pub struct Version<N: VersionNumber> {
    pub major: N,
    pub minor: N,
    pub patch: N,
}

impl<N: VersionNumber> Version<N> {
    pub fn new(major: N, minor: N, patch: N) -> Result<Self, NewVersionError> {
        let max = N::max();
        let min = N::min();
        if major == max {
            return Err(NewVersionError::MajorIsMax)
        } else if major == min {
            return Err(NewVersionError::MajorIsMin)
        }

        if minor == max {
            return Err(NewVersionError::MinorIsMax)
        } else if minor == min {
            return Err(NewVersionError::MinorIsMin)
        }

        if patch == max {
            return Err(NewVersionError::PatchIsMax)
        } else if patch == min {
            return Err(NewVersionError::PatchIsMin)
        }
        
        let ret = Version { major, minor, patch };
        Ok(ret)
    }

    pub fn try_new(major: N, minor: N, patch: N) -> Option<Self> {
        if !fast_compare(major, minor, patch) {
            return None;
        }

        Some(Self { major, minor, patch })
    }
}

impl<N: VersionNumber> PartialEq for Version<N> {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }

}

impl<N: VersionNumber> PartialOrd for Version<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let mut ordering = self.major.partial_cmp(&other.major).unwrap();
        if ordering.is_ne() {
            return Some(ordering);
        }
        ordering = self.minor.partial_cmp(&other.minor).unwrap();
        if ordering.is_ne() {
            return Some(ordering);
        }
        Some(self.patch.partial_cmp(&other.patch).unwrap())
    }
}