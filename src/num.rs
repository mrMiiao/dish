pub use core::num::*;
use core::ops::*;
use core::iter::{Iterator, Sum, Product};
use crate::fmt::{FormatNum, FormatInt};

macro_rules! impl_number {
    ($t:ty) => {
        impl Number for $t {
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MIN;
            const ZERO: Self = 0 as $t;
            const ONE: Self = 1 as $t;

            #[allow(unconditional_recursion)]
            #[inline]
            fn leading_zeros(self) -> u32 {
                <$t>::leading_zeros(self)
            }

            #[allow(unconditional_recursion)]
            #[inline]
            fn leading_ones(self) -> u32 {
                <$t>::leading_ones(self)
            }

            #[allow(unconditional_recursion)]
            #[inline]
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }

            #[allow(unconditional_recursion)]
            #[inline]
            fn trailing_ones(self) -> u32 {
                <$t>::trailing_ones(self)
            }
            
            #[inline]
            fn ilog(self, base: Self) -> u32 {
                (self as u128).ilog(base as u128)
            }

            #[inline]
            fn pow(self, exp: u32) -> Self {
                core::iter::repeat(self)
                    .take(exp as usize)
                    .product()
            }

            #[inline]
            fn ____internal__neg(self) -> Self {
                -(self as i128) as $t
            }

            #[inline]
            fn ____internal__log10(self) -> u32 {
                (self as u128).ilog10()
            }

            #[inline]
            fn ____internal__log2(self) -> u32 {
                (self as u128).ilog2()
            }

            #[inline]
            fn ____internal__asu128(self) -> u128 {
                self as u128
            }

            #[inline]
            fn ____internal__fromu128(n: u128) -> Self {
                n as $t
            }

            #[inline]
            fn ____internal__fromusize(n: usize) -> Self {
                n as $t
            }

            #[inline]
            fn ____internal__asusize(self) -> usize {
                self as usize
            }

            #[inline]
            fn ____internal__asu8(self) -> u8 {
                self as u8
            }

            #[inline]
            fn ____internal__asf64(self) -> f64 {
                self as f64
            }
        }
    };
}

macro_rules! impl_integer {
    ($t:ty) => {
        impl Integer for $t {}
    };
}

macro_rules! impl_sint {
    ($t:ty) => {
        impl SInt for $t {}
    };
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl UInt for $t {}
    };
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {}
    };
}

#[derive(Clone, Copy)]
pub struct Digits<T: Number> {
    n: T,
    flag: usize,
    len: usize,
}

impl<T: Number> Digits<T> {
    #[inline]
    pub fn new(n: T) -> Self {
        Self {
            n: n,
            flag: 0,
            len: n.____internal__log10() as usize + 1,
        }
    }

    #[inline]
    pub const fn number(self) -> T {
        self.n
    }
}

impl<T: Number> Iterator for Digits<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.flag < self.len {
            self.flag += 1;
            Some((self.n / T::____internal__fromusize(10usize.pow((self.len - self.flag) as u32)) % T::____internal__fromusize(10)).____internal__asu8())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Bit {
    Zero = 0u8,
    One = 1u8,
}

union BitRepr {
    b1: Bit,
    u: u8,
    b2: bool,
}

impl Bit {
    #[inline]
    pub fn from_u8(src: u8) -> Self {
        match src {
            0 => Self::Zero,
            _ => Self::One,
        }
    }

    #[inline]
    pub(crate) fn from_u8_unchecked(src: u8) -> Self {
        unsafe {
            BitRepr {u: src}.b1
        }
    }

    #[inline]
    pub const fn from_bool(src: bool) -> Self {
        unsafe {
            BitRepr {b2: src}.b1
        }
    }

    #[inline]
    pub const fn as_bool(self) -> bool {
        unsafe {
            BitRepr {b1: self}.b2
        }
    }

    #[inline]
    pub const fn as_u8(self) -> u8 {
        unsafe {
            BitRepr {b1: self}.u
        }
    }
}

impl core::ops::Deref for Bit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        unsafe {
            union RefRepr<'a> {
                b1: &'a Bit,
                u: &'a u8,
            }

            RefRepr {b1: self}.u
        }
    }
}

impl core::fmt::Display for Bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl core::ops::BitAnd for Bit {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_bool(self.as_bool() & rhs.as_bool())
    }
}

impl core::ops::BitOr for Bit {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bool(self.as_bool() & rhs.as_bool())
    }
}

impl core::ops::BitXor for Bit {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_bool(self.as_bool() ^ rhs.as_bool())
    }
}

impl core::ops::Not for Bit {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self::from_bool(!self.as_bool())
    }
}

/// Iterator over bits of a number.
#[derive(Clone, Copy)]
pub struct Bits<T: Number> {
    n: T,
    flag: usize,
    len: usize,
    zeros: u32,
}

impl<T: Number> Bits<T> {
    #[inline]
    pub fn new(n: T) -> Self {
        Self {
            n: n,
            flag: 0,
            len: n.____internal__log2() as usize + 1,
            zeros: n.leading_zeros(),
        }
    }

    #[inline]
    pub const fn number(self) -> T {
        self.n
    }
}

impl<T: Number> Iterator for Bits<T> {
    type Item = Bit;

    fn next(&mut self) -> Option<Bit> {
        if self.flag < self.len {
            self.flag += 1;
            Some(Bit::from_u8_unchecked((self.n / T::____internal__fromusize(2usize.pow((self.len - self.flag) as u32)) % T::____internal__fromusize(2)).____internal__asu8()))
        } else if !(self.flag < self.len) && self.zeros > 0 {
            self.zeros -= 1;
            Some(Bit::from_u8_unchecked(0))
        } else {
            None
        }
    }
}

/// General description of a number.
pub trait Number: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign + Clone + Copy + FormatNum + Sum<Self> + Product<Self> + PartialEq + PartialOrd {
    /// Least possible value.
    const MIN: Self;
    /// Biggest possible value.
    const MAX: Self;
    /// 0.
    const ZERO: Self;
    /// 1.
    const ONE: Self;
    /// Capacity in bits.
    const BITS: usize = core::mem::size_of::<Self>() * 8;

    /// Integer logarithm.
    fn ilog(self, base: Self) -> u32;

    /// Raises self to given power.
    fn pow(self, exp: u32) -> Self;
    
    /// Number of leading zeros.
    fn leading_zeros(self) -> u32;

    /// Number of leading ones.
    fn leading_ones(self) -> u32;

    /// Number of trailing zeros.
    fn trailing_zeros(self) -> u32;

    /// Number of trailing ones.
    fn trailing_ones(self) -> u32;

    /// Greatest count divisor of self and given number.
    #[inline]
    fn gcd(self, other: Self) -> Self {
        let (sm, om) = (self.____internal__asu128(), other.____internal__asu128());
        let min = if sm > om {om} else {sm};
        Self::____internal__fromu128((1..=min)
            .filter(|i| sm % i == 0 && om % i == 0)
            .max()
            .unwrap_or(1))
    }

    /// Least common multiple of self and given number.
    #[inline]
    fn lcm(self, other: Self) -> Self {
        (self * other / self.gcd(other)).module()
    }
    
    /// Reverse bits of self.
    #[inline]
    fn reverse_bits(self) -> Self {
        core::intrinsics::bitreverse(self)
    }

    /// Nth root of self.
    #[inline]
    fn root(self, n: u32) -> f64 {
        unsafe {core::intrinsics::powf64(self.____internal__asf64(), 1.0f64 / (n as f64))}
    }

    /// Iterator over digits (decimal).
    #[inline]
    fn digits(self) -> Digits<Self> {
        Digits::new(self)
    }

    /// Iterator over bits.
    #[inline]
    fn bits(self) -> Bits<Self> {
        Bits::new(self)
    }

    /// Absolute value of self.
    #[inline]
    fn module(self) -> Self {
        if self > Self::ZERO {
            self
        } else {
            self.____internal__neg()
        }
    }

    /// _.ilog(2)
    #[inline]
    fn ilog2(self) -> u32 {
        self.____internal__log2()
    }

    /// _.ilog(10)
    #[inline]
    fn ilog10(self) -> u32 {
        self.____internal__log10()
    }

    /// self < 0
    #[inline]
    fn is_negative(self) -> bool {
        self < Self::ZERO
    }

    #[doc(hidden)]
    fn ____internal__neg(self) -> Self;
    #[doc(hidden)]
    fn ____internal__log10(self) -> u32;
    #[doc(hidden)]
    fn ____internal__log2(self) -> u32;
    #[doc(hidden)]
    fn ____internal__asu128(self) -> u128;
    #[doc(hidden)]
    fn ____internal__fromusize(n: usize) -> Self;
    #[doc(hidden)]
    fn ____internal__asusize(self) -> usize;
    #[doc(hidden)]
    fn ____internal__asu8(self) -> u8;
    #[doc(hidden)]
    fn ____internal__asf64(self) -> f64;
    #[doc(hidden)]
    fn ____internal__fromu128(n: u128) -> Self;
}

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
impl_number!(usize);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(i128);
impl_number!(isize);
impl_number!(f32);
impl_number!(f64);

/// General description of an integer.
pub trait Integer: Number + FormatInt + Ord + Eq + BitAnd + BitOr + BitAnd + BitXor {}

impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);

/// General description of an unsigned integer.
pub trait UInt: Integer {}

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);
impl_uint!(usize);

/// General description of a signed integer.
pub trait SInt: Integer + Neg {}

impl_sint!(i8);
impl_sint!(i16);
impl_sint!(i32);
impl_sint!(i64);
impl_sint!(i128);
impl_sint!(isize);

/// General description of a floating point number.
pub trait Float: Number + Neg {}

impl_float!(f32);
impl_float!(f64);