use crate::*;

/// type Digit2 = BinaryDigit;
#[derive(Debug, PartialEq)]
pub enum BinaryDigit {
    /// 0
    Zero,
    /// 1
    One,
}

/// type Digit8 = Octal;
#[derive(Debug, PartialEq)]
pub enum OctalDigit {
    /// 0
    Zero,
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
}

/// type Digit10 = Decimal
#[derive(Debug, PartialEq)]
pub enum DecimalDigit {
    /// 0
    Zero,
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
}

/// type Digit16 = Hexadecimal
#[derive(Debug, PartialEq)]
pub enum HexadecimalDigit {
    /// 0
    Zero,
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
    /// a | A
    A,
    /// b | B
    B,
    /// c | C
    C,
    /// d | D
    D,
    /// e | E
    E,
    /// f | F
    F,
}
