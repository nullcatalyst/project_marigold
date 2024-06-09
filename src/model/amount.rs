use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Neg, Sub},
};

// A value that will be used to represent the amount of a currency.
// A currency amount should _never_ be represented as a floating point number, due to potentially accumulating rounding errors.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Amount(i64);

impl Amount {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let negative = s.starts_with('-');

        let s = if negative { &s[1..] } else { s };
        let mut parts = s.splitn(2, '.');

        // Unwrap will never panic here, because splitting an empty string will return an iterator with one empty string.
        let value: Result<i64, _> = parts.next().unwrap().parse();
        if value.is_err() {
            return None;
        }
        let value = value.unwrap();
        if value.checked_mul(10000).is_none() {
            // We store the fractional part as the lowest 4 digits of the integer.
            // If the integer part is too large, we can't store the fractional part.
            return None;
        }

        let frac: Result<u64, _> = parts.next().map_or(Ok(0), |s| s.parse());
        if frac.is_err() {
            return None;
        }
        let mut frac = frac.unwrap();
        while frac >= 10000 {
            // We have too many fractional decimal places.
            // Truncate the last digit until we have exactly 4.
            // Should this be rounded instead?
            frac /= 10;
        }

        if !negative {
            (10000 * value).checked_add(frac as i64).map(Self::new)
        } else {
            (-10000 * value).checked_sub(frac as i64).map(Self::new)
        }
    }

    pub fn to_string(&self) -> String {
        // Maximum length of a formatted number is 21 characters.
        // i64::max() == 9,223,372,036,854,775,807 (19 characters)
        // 19 + 1 (dot) + 1 (negative sign) = 21
        let mut s = String::with_capacity(21);
        if self.0 < 0 {
            s.push('-');
        }

        let (whole, frac) = (self.0 / 10000, self.0 % 10000);
        s.push_str(&whole.to_string());
        s.push('.');
        s.push_str(&format!("{:04}", frac.abs()));

        s
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

pub struct AmountOpError {
    pub lhs: Amount,
    pub rhs: Option<Amount>,
    pub op: &'static str,
}

impl Neg for Amount {
    type Output = Result<Self, AmountOpError>;

    fn neg(self) -> Result<Self, AmountOpError> {
        if let Some(value) = self.0.checked_neg() {
            Ok(Self::new(value))
        } else {
            Err(AmountOpError {
                lhs: self,
                rhs: None,
                op: "-",
            })
        }
    }
}

impl Add<Amount> for Amount {
    type Output = Result<Self, AmountOpError>;

    fn add(self, rhs: Self) -> Result<Self, AmountOpError> {
        if let Some(value) = self.0.checked_add(rhs.0) {
            Ok(Self::new(value))
        } else {
            Err(AmountOpError {
                lhs: self,
                rhs: Some(rhs),
                op: "+",
            })
        }
    }
}

impl Sub<Amount> for Amount {
    type Output = Result<Self, AmountOpError>;

    fn sub(self, rhs: Self) -> Result<Self, AmountOpError> {
        if let Some(value) = self.0.checked_sub(rhs.0) {
            Ok(Self::new(value))
        } else {
            Err(AmountOpError {
                lhs: self,
                rhs: Some(rhs),
                op: "-",
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_from_str() {
        assert_eq!(Amount::from_str("0"), Some(Amount(0))); // zero
        assert_eq!(Amount::from_str("123"), Some(Amount(123_0000))); // simple int
        assert_eq!(Amount::from_str("456.7891"), Some(Amount(456_7891))); // simple float
        assert_eq!(Amount::from_str("-987"), Some(Amount(-987_0000))); // simple negative
        assert_eq!(Amount::from_str("-1000.0001"), Some(Amount(-1000_0001))); // simple negative float
        assert_eq!(Amount::from_str("-0.0001"), Some(Amount(-0_0001))); // negative isn't lost, when integer part is zero
        assert_eq!(
            Amount::from_str("922337203685477.5807"),
            Some(Amount(922337203685477_5807))
        ); // max i64
        assert_eq!(
            Amount::from_str("-922337203685477.5808"),
            Some(Amount(-922337203685477_5808))
        ); // min i64

        assert_eq!(Amount::from_str("922337203685477.5808"), None); // overflow
        assert_eq!(Amount::from_str("duck"), None); // not a number
        assert_eq!(Amount::from_str("-duck.42"), None); // the duck put on a disguise
        assert_eq!(Amount::from_str("-42.duck"), None); // the duck tried a different disguise
    }
}
