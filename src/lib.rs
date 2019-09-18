#![no_std]

use core::fmt::{Display, Error, Formatter};
use heapless::consts::*;
use heapless::{String, Vec};

const HEX_CHARS: &[u8] = b"0123456789abcdef";

pub struct Eui48([u8; 6]);
pub struct Eui64([u8; 8]);

impl Eui48 {
    #[inline]
    pub fn to_string(&self) -> String<U12> {
        let mut to_string = Vec::new();

        for &byte in self.0.iter() {
            to_string.push(HEX_CHARS[(byte >> 4) as usize]).unwrap();
            to_string.push(HEX_CHARS[(byte & 0xf) as usize]).unwrap();
        }

        String::from_utf8(to_string).unwrap()
    }
}

impl Eui64 {
    #[inline]
    pub fn to_string(&self) -> String<U16> {
        let mut to_string = Vec::new();

        for &byte in self.0.iter() {
            to_string.push(HEX_CHARS[(byte >> 4) as usize]).unwrap();
            to_string.push(HEX_CHARS[(byte & 0xf) as usize]).unwrap();
        }

        String::from_utf8(to_string).unwrap()
    }
}

impl From<u64> for Eui48 {
    fn from(value: u64) -> Self {
        let b1: u8 = ((value >> 40) & 0xff) as u8;
        let b2: u8 = ((value >> 32) & 0xff) as u8;
        let b3: u8 = ((value >> 24) & 0xff) as u8;
        let b4: u8 = ((value >> 16) & 0xff) as u8;
        let b5: u8 = ((value >> 8) & 0xff) as u8;
        let b6: u8 = (value & 0xff) as u8;

        return Eui48([b1, b2, b3, b4, b5, b6]);
    }
}

impl From<u64> for Eui64 {
    fn from(value: u64) -> Self {
        Eui64(value.to_be_bytes())
    }
}

impl Display for Eui48 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Eui64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_eui48_to_string() {
    let eui48 = Eui48::from(85204980412143);

    assert_eq!(eui48.to_string(), "4d7e54972eef")
}

#[test]
fn test_eui64_display() {
    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(eui64.to_string(), "4d7e540000972eef")
}