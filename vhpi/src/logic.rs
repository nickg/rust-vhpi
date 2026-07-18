use std::convert::TryFrom;
use std::fmt;

use crate::Value;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitVal {
    /// Logic 0.
    Zero = vhpi_sys::vhpibit0 as u32,
    /// Logic 1.
    One = vhpi_sys::vhpibit1 as u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanVal {
    /// Logic false.
    False = vhpi_sys::vhpiFalse as u32,
    /// Logic true.
    True = vhpi_sys::vhpiTrue as u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Nine-value logic domain used by VHDL signals.
pub enum LogicVal {
    /// Uninitialized.
    U,
    /// Forcing unknown.
    X,
    /// Logic 0.
    Zero,
    /// Logic 1.
    One,
    /// High impedance.
    Z,
    /// Weak unknown.
    W,
    /// Weak 0.
    L,
    /// Weak 1.
    H,
    /// Don't care.
    DontCare,
}

impl From<u8> for LogicVal {
    fn from(val: u8) -> Self {
        match u32::from(val) {
            vhpi_sys::vhpiU => LogicVal::U,
            vhpi_sys::vhpiX => LogicVal::X,
            vhpi_sys::vhpi0 => LogicVal::Zero,
            vhpi_sys::vhpi1 => LogicVal::One,
            vhpi_sys::vhpiZ => LogicVal::Z,
            vhpi_sys::vhpiW => LogicVal::W,
            vhpi_sys::vhpiL => LogicVal::L,
            vhpi_sys::vhpiH => LogicVal::H,
            vhpi_sys::vhpiDontCare => LogicVal::DontCare,
            _ => LogicVal::X, // Treat unknown values as X
        }
    }
}

impl From<LogicVal> for vhpi_sys::vhpiEnumT {
    fn from(logic: LogicVal) -> Self {
        match logic {
            LogicVal::U => vhpi_sys::vhpiU,
            LogicVal::X => vhpi_sys::vhpiX,
            LogicVal::Zero => vhpi_sys::vhpi0,
            LogicVal::One => vhpi_sys::vhpi1,
            LogicVal::Z => vhpi_sys::vhpiZ,
            LogicVal::W => vhpi_sys::vhpiW,
            LogicVal::L => vhpi_sys::vhpiL,
            LogicVal::H => vhpi_sys::vhpiH,
            LogicVal::DontCare => vhpi_sys::vhpiDontCare,
        }
    }
}

impl From<LogicVal> for u8 {
    fn from(logic: LogicVal) -> Self {
        match logic {
            LogicVal::U => b'u',
            LogicVal::X => b'x',
            LogicVal::Zero => b'0',
            LogicVal::One => b'1',
            LogicVal::Z => b'z',
            LogicVal::W => b'w',
            LogicVal::L => b'l',
            LogicVal::H => b'h',
            LogicVal::DontCare => b'-',
        }
    }
}

impl From<char> for LogicVal {
    fn from(c: char) -> Self {
        match c {
            'U' | 'u' => LogicVal::U,
            'X' | 'x' => LogicVal::X,
            '0' => LogicVal::Zero,
            '1' => LogicVal::One,
            'Z' | 'z' => LogicVal::Z,
            'W' | 'w' => LogicVal::W,
            'L' | 'l' => LogicVal::L,
            'H' | 'h' => LogicVal::H,
            '-' => LogicVal::DontCare,
            _ => LogicVal::X,
        }
    }
}

impl fmt::Display for LogicVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogicVal::U => "U",
                LogicVal::X => "X",
                LogicVal::Zero => "0",
                LogicVal::One => "1",
                LogicVal::Z => "Z",
                LogicVal::W => "W",
                LogicVal::L => "L",
                LogicVal::H => "H",
                LogicVal::DontCare => "-",
            }
        )
    }
}

/// A VHDL logic vector stored in most-significant-bit-first order.
///
/// `LogicVec` is used to represent scalar and vector values made up of
/// [`LogicVal`] elements. Integer constructors and conversions interpret the
/// first element as the most significant bit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicVec {
    data: Vec<LogicVal>,
}

impl LogicVec {
    /// Creates a logic vector by copying the provided slice of logic values.
    #[must_use]
    pub fn new(data: impl AsRef<[LogicVal]>) -> Self {
        Self {
            data: data.as_ref().to_vec(),
        }
    }

    /// Returns an iterator over the vector's logic values in stored order.
    pub fn iter(&self) -> impl Iterator<Item = &LogicVal> {
        self.data.iter()
    }

    /// Creates a logic vector from a string of VHDL logic symbols.
    ///
    /// The input is stored in the same left-to-right order as the string,
    /// which means the first character becomes the most significant bit for
    /// integer-style interpretations.
    ///
    /// If the string contains any invalid characters, `None` is returned.
    #[must_use]
    pub fn try_from_str(s: &str) -> Option<Self> {
        let mut bits = Vec::with_capacity(s.len());
        for c in s.chars() {
            let bit = match c {
                '0' => LogicVal::Zero,
                '1' => LogicVal::One,
                'U' | 'u' => LogicVal::U,
                'W' | 'w' => LogicVal::W,
                'X' | 'x' => LogicVal::X,
                'Z' | 'z' => LogicVal::Z,
                'H' | 'h' => LogicVal::H,
                'L' | 'l' => LogicVal::L,
                '-' => LogicVal::DontCare,
                _ => return None, // Invalid character
            };
            bits.push(bit);
        }
        Some(Self { data: bits })
    }

    /// Creates a logic vector from a string of logic symbols.
    ///
    /// If the string contains any invalid characters, the invalid characters are treated as `LogicVal::X`.
    #[must_use]
    fn from_str(s: &str) -> Self {
        let mut data = Vec::with_capacity(s.len());
        for c in s.chars() {
            data.push(LogicVal::from(c));
        }
        Self { data }
    }

    /// Returns the number of logic values in the vector.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the vector contains no logic values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Builds a logic vector from a signed integer using two's complement bits.
    ///
    /// The resulting vector is stored most-significant-bit first.
    #[must_use]
    pub fn from_int(value: impl Into<i64>, width: usize) -> Self {
        assert!(
            width <= 64,
            "width must be <= 64 for signed integer conversion"
        );
        let mut data = Vec::with_capacity(width);
        let value: i64 = value.into();
        for i in 0..width {
            let bit = (value >> i) & 1;
            data.push(if bit == 0 {
                LogicVal::Zero
            } else {
                LogicVal::One
            });
        }
        data.reverse();
        Self { data }
    }

    /// Builds a logic vector from an unsigned integer.
    ///
    /// The resulting vector is stored most-significant-bit first.
    #[must_use]
    pub fn from_uint(value: impl Into<u64>, width: usize) -> Self {
        assert!(
            width <= 64,
            "width must be <= 64 for unsigned integer conversion"
        );
        let value: u64 = value.into();
        let mut data = Vec::with_capacity(width);
        for i in 0..width {
            let bit = (value >> i) & 1;
            data.push(if bit == 0 {
                LogicVal::Zero
            } else {
                LogicVal::One
            });
        }
        data.reverse();
        Self { data }
    }

    /// Returns the underlying logic values as a slice in stored order.
    #[must_use]
    pub fn as_slice(&self) -> &[LogicVal] {
        &self.data
    }

    #[cfg(feature = "bigint")]
    #[must_use]
    /// Builds a logic vector from a `BigInt` using two's complement bits.
    pub fn from_bigint(value: &num_bigint::BigInt, width: usize) -> Self {
        let mut data = Vec::with_capacity(width);
        for i in 0..width {
            let bit = value.bit(i as u64);
            data.push(if bit { LogicVal::One } else { LogicVal::Zero });
        }
        data.reverse();
        Self { data }
    }

    #[cfg(feature = "bigint")]
    #[must_use]
    /// Builds a logic vector from a `BigUint` using its bits.
    pub fn from_biguint(value: &num_bigint::BigUint, width: usize) -> Self {
        let mut data = Vec::with_capacity(width);
        for i in 0..width {
            let bit = value.bit(i as u64);
            data.push(if bit { LogicVal::One } else { LogicVal::Zero });
        }
        data.reverse();
        Self { data }
    }

    /// Creates a logic vector from raw VHPI enum values.
    #[must_use]
    pub(crate) fn from_slice(slice: impl AsRef<[u32]>) -> Self {
        let slice = slice.as_ref();
        let mut logic_data = Vec::with_capacity(slice.len());
        for &val in slice {
            logic_data.push(LogicVal::from(val as u8));
        }
        Self { data: logic_data }
    }

    /// Wraps this logic vector as a [`Value::LogicVec`].
    #[must_use]
    pub fn as_value(&self) -> Value {
        Value::LogicVec(self.clone())
    }

    /// Returns a copy of the vector with its element order reversed.
    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut reversed_data = self.data.clone();
        reversed_data.reverse();
        Self {
            data: reversed_data,
        }
    }

    #[cfg(feature = "bigint")]
    #[must_use]
    /// Converts this logic vector into a `BigInt` if it contains only binary values.
    pub fn as_bigint(&self) -> Option<num_bigint::BigInt> {
        let mut value = num_bigint::BigInt::from(0u8);
        let one = num_bigint::BigInt::from(1u8);
        for &logic_val in &self.data {
            value <<= 1;
            match logic_val {
                LogicVal::Zero => {}
                LogicVal::One => value |= &one,
                _ => return None,
            }
        }

        if matches!(self.data.first(), Some(LogicVal::One)) && !self.data.is_empty() {
            let sign_base = &one << self.data.len();
            value -= sign_base;
        }

        Some(value)
    }

    #[cfg(feature = "bigint")]
    #[must_use]
    /// Converts this logic vector into a `BigUint` if it contains only binary values.
    pub fn as_biguint(&self) -> Option<num_bigint::BigUint> {
        let mut value = num_bigint::BigUint::from(0u8);
        let one = num_bigint::BigUint::from(1u8);
        for &logic_val in &self.data {
            value <<= 1;
            match logic_val {
                LogicVal::Zero => {}
                LogicVal::One => value |= &one,
                _ => return None,
            }
        }
        Some(value)
    }
}

impl std::fmt::Display for LogicVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .data
            .iter()
            .map(|val| char::from(u8::from(*val)))
            .collect();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicVecToIntError {
    TooManyBits,
    InvalidSymbol,
}

macro_rules! impl_try_from_logic_vec_for_ints {
    ($(($signed:ty, $unsigned:ty, $bits:expr)),+ $(,)?) => {
        $(
            impl TryFrom<LogicVec> for $signed {
                type Error = LogicVecToIntError;

                fn try_from(value: LogicVec) -> Result<Self, Self::Error> {
                    let width = value.data.len();
                    if width > $bits {
                        return Err(LogicVecToIntError::TooManyBits);
                    }

                    let is_negative = matches!(value.data.first(), Some(LogicVal::One));
                    let mut out: $signed = 0;
                    for &logic_val in &value.data {
                        out <<= 1;
                        match logic_val {
                            LogicVal::Zero => {}
                            LogicVal::One => out |= 1,
                            _ => return Err(LogicVecToIntError::InvalidSymbol),
                        }
                    }

                    if is_negative && width < $bits {
                        out |= (!0 as $signed) << width;
                    }

                    Ok(out)
                }
            }

            impl TryFrom<LogicVec> for $unsigned {
                type Error = LogicVecToIntError;

                fn try_from(value: LogicVec) -> Result<Self, Self::Error> {
                    if value.data.len() > $bits {
                        return Err(LogicVecToIntError::TooManyBits);
                    }

                    let mut out: $unsigned = 0;
                    for &logic_val in &value.data {
                        out <<= 1;
                        match logic_val {
                            LogicVal::Zero => {}
                            LogicVal::One => out |= 1,
                            _ => return Err(LogicVecToIntError::InvalidSymbol),
                        }
                    }

                    Ok(out)
                }
            }
        )+
    };
}

impl_try_from_logic_vec_for_ints!(
    (i8, u8, 8),
    (i16, u16, 16),
    (i32, u32, 32),
    (i64, u64, 64),
    (i128, u128, 128),
);

impl From<&str> for LogicVec {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<&String> for LogicVec {
    fn from(s: &String) -> Self {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{LogicVal, LogicVec, LogicVecToIntError};
    use crate::Value;

    #[test]
    fn logic_val_to_u8_uses_lowercase_symbols() {
        assert_eq!(u8::from(LogicVal::U), b'u');
        assert_eq!(u8::from(LogicVal::X), b'x');
        assert_eq!(u8::from(LogicVal::Zero), b'0');
        assert_eq!(u8::from(LogicVal::One), b'1');
        assert_eq!(u8::from(LogicVal::Z), b'z');
        assert_eq!(u8::from(LogicVal::W), b'w');
        assert_eq!(u8::from(LogicVal::L), b'l');
        assert_eq!(u8::from(LogicVal::H), b'h');
        assert_eq!(u8::from(LogicVal::DontCare), b'-');
    }

    #[test]
    fn logic_vec_new_from_str_parses_known_symbols() {
        let vec = LogicVec::from("uX01zWlH-");

        assert_eq!(vec.len(), 9);
        assert_eq!(
            vec.as_slice(),
            &[
                LogicVal::U,
                LogicVal::X,
                LogicVal::Zero,
                LogicVal::One,
                LogicVal::Z,
                LogicVal::W,
                LogicVal::L,
                LogicVal::H,
                LogicVal::DontCare,
            ]
        );
    }

    #[test]
    fn logic_vec_new_from_str_rejects_invalid_symbols() {
        assert_eq!(
            LogicVec::from("10A1").as_slice(),
            &[LogicVal::One, LogicVal::Zero, LogicVal::X, LogicVal::One]
        );
    }

    #[test]
    fn logic_vec_reverse_returns_reversed_copy() {
        let original = LogicVec::new(vec![LogicVal::Zero, LogicVal::One, LogicVal::X]);

        let reversed = original.reverse();
        assert_eq!(
            reversed.as_slice(),
            &[LogicVal::X, LogicVal::One, LogicVal::Zero]
        );
        assert_eq!(
            original.as_slice(),
            &[LogicVal::Zero, LogicVal::One, LogicVal::X]
        );
    }

    #[test]
    fn logic_vec_as_value_wraps_as_value_logic_vec() {
        let vec = LogicVec::new(vec![LogicVal::Zero, LogicVal::One]);

        assert_eq!(vec.as_value(), Value::LogicVec(vec));
    }

    #[test]
    fn logic_vec_as_i64_rejects_non_binary_and_too_wide_vectors() {
        let non_binary = LogicVec::new(vec![LogicVal::One, LogicVal::X, LogicVal::Zero]);
        let too_wide = LogicVec::new(vec![LogicVal::Zero; 65]);

        assert!(TryInto::<i64>::try_into(non_binary).is_err());
        assert!(TryInto::<i64>::try_into(too_wide).is_err());
    }

    #[test]
    fn logic_vec_try_from_i16_sign_extends_binary_value() {
        let value = LogicVec::new(vec![LogicVal::One, LogicVal::Zero]);

        assert_eq!(i16::try_from(value), Ok(-2));
    }

    #[test]
    fn logic_vec_try_from_u16_parses_binary_value() {
        let value = LogicVec::new(vec![LogicVal::One, LogicVal::Zero, LogicVal::One]);

        assert_eq!(u16::try_from(value), Ok(5));
    }

    #[test]
    fn logic_vec_try_from_rejects_non_binary_values() {
        let value = LogicVec::new(vec![LogicVal::One, LogicVal::X, LogicVal::Zero]);

        assert_eq!(u8::try_from(value), Err(LogicVecToIntError::InvalidSymbol));
    }

    #[test]
    fn logic_vec_try_from_rejects_values_wider_than_target() {
        let too_wide = LogicVec::new(vec![LogicVal::Zero; 9]);

        assert_eq!(u8::try_from(too_wide), Err(LogicVecToIntError::TooManyBits));
    }

    #[test]
    fn logic_vec_try_from_u8_uses_msb_first_non_symmetric_pattern() {
        let value = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);

        assert_eq!(u8::try_from(value), Ok(38));
    }

    #[test]
    fn logic_vec_try_from_i8_positive_uses_msb_first_non_symmetric_pattern() {
        let value = LogicVec::new(vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
        ]);

        assert_eq!(i8::try_from(value), Ok(11));
    }

    #[test]
    fn logic_vec_try_from_i8_negative_sign_extends_msb_first_pattern() {
        let value = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);

        assert_eq!(i8::try_from(value), Ok(-19));
    }

    #[test]
    fn logic_vec_try_from_i32_negative_uses_msb_first_non_symmetric_pattern() {
        let value = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);

        assert_eq!(i32::try_from(value), Ok(-1627));
    }

    #[test]
    fn logic_vec_try_from_u16_rejects_too_many_bits_with_explicit_error() {
        let too_wide = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);

        assert_eq!(
            u16::try_from(too_wide),
            Err(LogicVecToIntError::TooManyBits)
        );
    }

    #[test]
    fn logic_vec_try_from_i16_rejects_invalid_symbol_with_explicit_error() {
        let value = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::DontCare,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);

        assert_eq!(i16::try_from(value), Err(LogicVecToIntError::InvalidSymbol));
    }

    #[test]
    fn logic_vec_new_from_signed_integer_produces_msb_first_order() {
        let vec = LogicVec::from_int(-3, 4);

        assert_eq!(
            vec.as_slice(),
            &[LogicVal::One, LogicVal::One, LogicVal::Zero, LogicVal::One]
        );
    }

    #[test]
    fn logic_vec_from_int_round_trips_back_to_i8() {
        let vec = LogicVec::from_int(-19i8, 6);

        assert_eq!(i8::try_from(vec), Ok(-19));
    }

    #[test]
    fn logic_vec_from_int_round_trips_back_to_i64() {
        let vec = LogicVec::from_int(-3_211_457i64, 23);

        assert_eq!(i64::try_from(vec), Ok(-3_211_457));
    }

    #[test]
    fn logic_vec_new_from_unsigned_integer_produces_msb_first_order() {
        let vec = LogicVec::from_uint(0b1101u8, 4);

        assert_eq!(
            vec.as_slice(),
            &[LogicVal::One, LogicVal::One, LogicVal::Zero, LogicVal::One]
        );
    }

    #[test]
    fn logic_vec_from_uint_round_trips_back_to_u8() {
        let vec = LogicVec::from_uint(38u8, 6);

        assert_eq!(u8::try_from(vec), Ok(38));
    }

    #[test]
    fn logic_vec_from_uint_round_trips_back_to_u64() {
        let vec = LogicVec::from_uint(0x12_34_56_78_9Au64, 41);

        assert_eq!(u64::try_from(vec), Ok(0x12_34_56_78_9A));
    }

    #[test]
    fn logic_vec_from_slice_maps_vhpi_constants() {
        let vec = LogicVec::from_slice([vhpi_sys::vhpi0, vhpi_sys::vhpi1, vhpi_sys::vhpiDontCare]);

        assert_eq!(
            vec.as_slice(),
            &[LogicVal::Zero, LogicVal::One, LogicVal::DontCare]
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_as_bigint_handles_signed_values() {
        let neg_three = LogicVec::new(vec![
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);
        let pos_one = LogicVec::new(vec![LogicVal::Zero, LogicVal::Zero, LogicVal::One]);

        assert_eq!(neg_three.as_bigint(), Some(num_bigint::BigInt::from(-3i8)));
        assert_eq!(pos_one.as_bigint(), Some(num_bigint::BigInt::from(1u8)));
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_biguint_round_trip_for_wide_value() {
        let original = (num_bigint::BigUint::from(1u8) << 130)
            | (num_bigint::BigUint::from(1u8) << 7)
            | num_bigint::BigUint::from(0x2Du8);

        let vec = LogicVec::from_biguint(&original, 131);

        assert_eq!(vec.as_biguint(), Some(original));
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_bigint_round_trip_for_negative_value() {
        let original = num_bigint::BigInt::from(-12_345i32);

        let vec = LogicVec::from_bigint(&original, 32);

        assert_eq!(vec.as_bigint(), Some(original));
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_new_from_bigint_matches_twos_complement_bits() {
        let vec = LogicVec::from_bigint(&num_bigint::BigInt::from(-3i8), 4);

        assert_eq!(
            vec.as_slice(),
            &[LogicVal::One, LogicVal::One, LogicVal::Zero, LogicVal::One]
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_new_from_biguint_matches_binary_bits() {
        let vec = LogicVec::from_biguint(&num_bigint::BigUint::from(13u8), 4);

        assert_eq!(
            vec.as_slice(),
            &[LogicVal::One, LogicVal::One, LogicVal::Zero, LogicVal::One]
        );
    }
}
