use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::LazyLock;
use aocrs::geo::pos::PosIdx;

static NUMPAD: LazyLock<Vec<Vec<Option<u8>>>> = LazyLock::new(|| {
    vec![
        vec![Some(b'7'), Some(b'8'), Some(b'9')],
        vec![Some(b'4'), Some(b'5'), Some(b'6')],
        vec![None, Some(b'0'), Some(b'A')],
    ]
});

static DIRPAD: LazyLock<Vec<Vec<Option<u8>>>> = LazyLock::new(|| {
    vec![
        vec![None, Some(b'^'), Some(b'A')],
        vec![Some(b'<'), Some(b'v'), Some(b'>')],
    ]
});

static NUMPAD_POS: LazyLock<HashMap<PosIdx, u8>> = LazyLock::new(|| {
    HashMap::from([
        (PosIdx::new(0, 0), b'7'),
        (PosIdx::new(1, 0), b'8'),
        (PosIdx::new(2, 0), b'9'),
        (PosIdx::new(0, 1), b'4'),
        (PosIdx::new(1, 1), b'5'),
        (PosIdx::new(2, 1), b'6'),
        (PosIdx::new(0, 2), b'1'),
        (PosIdx::new(1, 2), b'2'),
        (PosIdx::new(2, 2), b'3'),
        (PosIdx::new(1, 3), b'0'),
        (PosIdx::new(2, 3), b'A'),
    ])
});

static DIRPAD_POS: LazyLock<HashMap<PosIdx, u8>> = LazyLock::new(|| {
    HashMap::from([
        (PosIdx::new(1, 0), b'^'),
        (PosIdx::new(2, 0), b'A'),
        (PosIdx::new(1, 1), b'<'),
        (PosIdx::new(2, 1), b'v'),
        (PosIdx::new(1, 2), b'>'),
    ])
});

#[derive(Clone, Debug)]
pub struct Code {
    value: String,
    numeric: i32,
}

impl FromStr for Code {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = String::from(s);
        let numeric = i32::from_str(&s[0..3])?;
        Ok(Self { value, numeric })
    }
}

impl Code {
    pub fn complexity(&self) -> usize {
        0
    }
}
