
use std::fmt;
use std::ops::{
    Add, Sub, Div, Mul,
    Neg, Rem
};

use std::cmp::{
    PartialEq,
    PartialOrd,
    Ordering
};

// TODO handle all variants + check for overflowing

#[derive(Debug, Clone, Copy)]
pub enum DigitType { // might capitalise these types later
    u8(u8), u16(u16), u32(u32), u64(u64),
    i8(i8), i16(i16), i32(i32), i64(i64),
                      f32(f32), f64(f64)
}

impl DigitType {
    pub fn from_string(s: String) -> Self {
        if let Ok(v) = s.parse::<u8>() { Self::u8(v) }
        else if let Ok(v) = s.parse::<u16>() { Self::u16(v) }
        else if let Ok(v) = s.parse::<u32>() { Self::u32(v) }
        else if let Ok(v) = s.parse::<u64>() { Self::u64(v) }

        else if let Ok(v) = s.parse::<i32>() { Self::i32(v) }
        else if let Ok(v) = s.parse::<f64>() { Self::f64(v) }
        else { panic!("DigitType from_string error")  }
    }

    // this section has to be todod better later i dont like this and cannot think properly
    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => {
                let e = u32::pow(a as u32, b as u32);
                DigitType::u32(e)
            },
            (DigitType::u8(a), DigitType::i8(b)) => {
                let e = u32::pow(a as u32, b as u32);
                DigitType::u32(e)
            },
            _ => panic!("Incompatible types for exponentiation"),
        }
    }

    pub fn root(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => {
                let e = (1f32 / a as f32);
                DigitType::f32(f32::powf(b as f32, e))
            },
            (DigitType::u8(a), DigitType::i8(b)) => {
                let e = (1f32 / b as f32);
                DigitType::f32(f32::powf(a as f32, e))
            },
            (a,b) => panic!("Incompatible types for root for: {a:?} AND {b:?}"),
        }
    }

    pub fn sqrt(self) -> Self {
        match self {
            DigitType::u8(a) => DigitType::u8(a.isqrt()),
            _ => panic!("Incompatible types for square rooting"),
        }
    }
}

impl fmt::Display for DigitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // unsigned
            Self::u8(v) => write!(f, "{v}"),
            Self::u16(v) => write!(f, "{v}"),
            Self::u32(v) => write!(f, "{v}"),
            Self::u64(v) => write!(f, "{v}"),
            // signed
            Self::i8(v) => write!(f, "{v}"),
            Self::i16(v) => write!(f, "{v}"),
            Self::i32(v) => write!(f, "{v}"),
            Self::i64(v) => write!(f, "{v}"),
            // float
            Self::f32(v) => write!(f, "{v}"),
            Self::f64(v) => write!(f, "{v}"),
            v => panic!("Incomplete type writing/printing for {v}")
        }
    }
}

// swap the below to something similar like this
// impl<'a, 'b> Add<&'b Vector> for &'a Vector {
//     type Output = Vector;

//     fn add(self, other: &'b Vector) -> Vector {
//         Vector {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

impl Add for DigitType {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => {
                let c = a.checked_add(b);
                if let Some(res) = c {
                    DigitType::u8(res)
                } else {
                    DigitType::u16(a as u16 + b as u16)
                }
            },
            (DigitType::u16(a), DigitType::u16(b)) => {
                let c = a.checked_add(b);
                if let Some(res) = c {
                    DigitType::u16(res)
                } else {
                    DigitType::u32(a as u32 + b as u32)
                }
            },
            (DigitType::u32(a), DigitType::u32(b)) => DigitType::u32(a + b),
            (DigitType::u64(a), DigitType::u64(b)) => DigitType::u64(a + b),

            (DigitType::u16(a), DigitType::u8(b)) => {
                let c = a.checked_add(b as u16);
                if let Some(res) = c {
                    DigitType::u16(res)
                } else {
                    DigitType::u32(a as u32 + b as u32)
                }
            },
            (DigitType::u32(a), DigitType::u8(b)) => DigitType::u32(a + b as u32),

            (DigitType::i32(a), DigitType::i32(b)) => DigitType::i32(a + b),
            (DigitType::i8(a), DigitType::i32(b)) => DigitType::i32(a as i32 + b),
            (DigitType::i32(a), DigitType::i8(b)) => DigitType::i32(a + b as i32),
            _ => panic!("Incompatible types for addition"),
        }
    }
}

impl Sub for DigitType {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => DigitType::u8(a - b),
            (DigitType::i32(a), DigitType::i32(b)) => DigitType::i32(a - b),

            (DigitType::f64(a), DigitType::f64(b)) => DigitType::f64(a - b),
            (a,b) => panic!("Incompatible types for subtraction: {a} AND {b}"),
        }
    }
}

impl Mul for DigitType {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => DigitType::u8(a * b),
            (DigitType::i32(a), DigitType::i32(b)) => DigitType::i32(a * b),
            _ => panic!("Incompatible types for multiplication"),
        }
    }
}

impl Div for DigitType {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => DigitType::u8(a / b),
            (DigitType::i32(a), DigitType::i32(b)) => DigitType::i32(a / b),
            _ => panic!("Incompatible types for division"),
        }
    }
}

impl Neg for DigitType {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            //DigitType::u8(x) => DigitType::u8(-x),
            DigitType::i32(x) => DigitType::i32(-x),
            _ => panic!("Incompatible types for negation"),
        }
    }
}

impl Rem for DigitType {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DigitType::u8(a), DigitType::u8(b)) => DigitType::u8(a % b),
            (DigitType::i32(a), DigitType::i32(b)) => DigitType::i32(a % b),
            _ => panic!("Incompatible types for remainders/modulo"),
        }
    }
}

impl PartialEq for DigitType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DigitType::u8(a), DigitType::u8(b)) => a == b,
            (DigitType::i32(a), DigitType::i32(b)) => a == b,
            _ => panic!("Incompatible types for equality"),
        }
    }
}

impl PartialOrd for DigitType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (DigitType::u8(a), DigitType::u8(b)) => a <= b,
            (DigitType::u16(a), DigitType::u16(b)) => a <= b,
            (DigitType::u32(a), DigitType::u32(b)) => a <= b,
            (DigitType::u64(a), DigitType::u64(b)) => a <= b,

            (DigitType::u8(a), DigitType::u16(b)) => &(*a as u16) <= b,
            (DigitType::u16(a), DigitType::u8(b)) => a <= &(*b as u16),
            (DigitType::u8(a), DigitType::u32(b)) => &(*a as u32) <= b,
            (DigitType::u32(a), DigitType::u8(b)) => a <= &(*b as u32),

            (DigitType::u16(a), DigitType::u32(b)) => &(*a as u32) <= b,
            (DigitType::u32(a), DigitType::u16(b)) => a <= &(*b as u32),

            (DigitType::u8(a), DigitType::u32(b)) => &(*a as u32) <= b,
            (DigitType::u32(a), DigitType::u8(b)) => a <= &(*b as u32),


            (DigitType::u32(a), DigitType::u32(b)) => a <= b,


            (DigitType::u8(a), DigitType::u8(b)) => a <= b,

            (DigitType::i32(a), DigitType::i32(b)) => a <= b,

            (a, b) => panic!("Incompatible types for ord: {a:?} AND {b:?}"),
        }
    }
}
