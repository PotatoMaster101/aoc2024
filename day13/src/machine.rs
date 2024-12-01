use std::str::FromStr;
use std::sync::LazyLock;
use aocrs::geo::pos::Pos;
use regex::Regex;

static RGX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d+").unwrap()
});

#[derive(Clone, Debug)]
pub struct Machine {
    button_a: Pos<i64>,
    button_b: Pos<i64>,
    prize: Pos<i64>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let button_a: Vec<_> = RGX
            .find_iter(lines[0])
            .filter_map(|m| i64::from_str(m.as_str()).ok())
            .collect();
        let button_b: Vec<_> = RGX
            .find_iter(lines[1])
            .filter_map(|m| i64::from_str(m.as_str()).ok())
            .collect();
        let prize: Vec<_> = RGX
            .find_iter(lines[2])
            .filter_map(|m| i64::from_str(m.as_str()).ok())
            .collect();

        Ok(Self {
            button_a: Pos::new(button_a[0], button_a[1]),
            button_b: Pos::new(button_b[0], button_b[1]),
            prize: Pos::new(prize[0], prize[1]),
        })
    }
}

impl Machine {
    pub fn win_tokens(&self, offset: i64) -> Option<i64> {
        let top = Pos::new(self.button_a.x, self.button_b.x);
        let bottom = Pos::new(self.button_a.y, self.button_b.y);
        let prize = Pos::new(self.prize.x + offset, self.prize.y + offset);
        Self::cramers(&top, &bottom, &prize)
    }

    fn cramers(top: &Pos<i64>, bottom: &Pos<i64>, prize: &Pos<i64>) -> Option<i64> {
        let coefficient = Self::determinant(top, bottom);
        let x_mat = Self::determinant(&Pos::new(prize.x, top.y), &Pos::new(prize.y, bottom.y));
        let y_mat = Self::determinant(&Pos::new(top.x, prize.x), &Pos::new(bottom.x, prize.y));
        if x_mat % coefficient == 0 && y_mat % coefficient == 0 {
            Some((x_mat / coefficient) * 3 + (y_mat / coefficient))
        } else {
            None
        }
    }

    #[inline]
    fn determinant(top: &Pos<i64>, bottom: &Pos<i64>) -> i64 {
        top.x * bottom.y - top.y * bottom.x
    }
}
