use std::{marker::PhantomData, convert::TryFrom, borrow::Borrow, fmt::Display};
use crate::aligned_bytes::{Misaligned, Alignment};

/// Represents a usize that is some multiple of Alignment::ALIGNMENT.
///
/// `AlignedOffset<A2>` is essentially a usize that is multiple of 2.  This is
/// useful because you can slice a `AlignedSlice<A2>` and be statically
/// guaranteed that the slice will still be aligned.
///
/// Use `try_into()` or `align_offset` to construct values of this type.
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct AlignedOffset<A:Alignment>(usize, PhantomData<A>);
impl<A:Alignment> AlignedOffset<A> {
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

/// Construct an `AlignedOffset` by rounding-up idx until it's a multiple of
/// `A::ALIGNMENT`.
///
/// This is useful for GVariant deserialisation because often we have an offset
/// representing the end of a value and we want to find the start of the next
/// one.  This involves padding according to the next value's alignment.
pub fn align_offset<A: Alignment>(idx : usize) -> AlignedOffset<A> {
    AlignedOffset::<A>((idx + A::ALIGNMENT - 1) & !(A::ALIGNMENT - 1),
                       PhantomData::<A>{})
}

impl<A:Alignment> Display for AlignedOffset<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<A:Alignment> PartialEq<usize> for AlignedOffset<A> {
    fn eq(&self, other: &usize) -> bool {
        *other == self.0
    }
}
impl<A:Alignment> PartialOrd<usize> for AlignedOffset<A> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}
impl<A:Alignment> PartialEq<AlignedOffset<A>> for usize {
    fn eq(&self, other: &AlignedOffset<A>) -> bool {
        *self == other.0
    }
}
impl<A:Alignment> PartialOrd<AlignedOffset<A>> for usize {
    fn partial_cmp(&self, other: &AlignedOffset<A>) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}
impl<A:Alignment> TryFrom<usize> for AlignedOffset<A>
{
    type Error = Misaligned;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value % A::ALIGNMENT == 0 {
            Ok(Self(value, PhantomData::<A>{}))
        } else {
            Err(Misaligned{})
        }
    }
}
impl<A:Alignment> Borrow<usize> for AlignedOffset<A>
{
    fn borrow(&self) -> &usize {
        &self.0
    }
}
impl<A:Alignment> From<AlignedOffset<A>> for usize
{
    fn from(value: AlignedOffset<A>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aligned_bytes::{A2, A4};
    use std::convert::TryInto;

    #[test]
    fn test() {
        let x : Result<AlignedOffset<A2>, Misaligned> = 3usize.try_into();
        assert!(x.is_err());

        let x : AlignedOffset<A2> = 0usize.try_into().unwrap();
        assert_eq!(x.to_usize(), 0usize);

        let x : AlignedOffset<A4> = 8usize.try_into().unwrap();
        assert_eq!(x.to_usize(), 8usize);

        assert_eq!(align_offset::<A4>(0).to_usize(), 0);
        assert_eq!(align_offset::<A4>(1).to_usize(), 4);
        assert_eq!(align_offset::<A4>(4).to_usize(), 4);
        assert_eq!(align_offset::<A4>(6).to_usize(), 8);
        assert_eq!(align_offset::<A4>(7).to_usize(), 8);
        assert_eq!(align_offset::<A4>(8).to_usize(), 8);
        assert_eq!(align_offset::<A4>(9).to_usize(), 12);
    }
}