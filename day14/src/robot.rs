use std::collections::HashSet;
use std::str::FromStr;
use aocrs::geo::area::Area;
use aocrs::geo::direction::Direction;
use aocrs::geo::pos::Pos;

const MAX_ITER: usize = 20000;

#[derive(Clone, Copy, Debug)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle,
}

impl Quadrant {
    fn from_pos(pos: &Pos<i64>, mid_x: i64, mid_y: i64) -> Self {
        if pos.x == mid_x || pos.y == mid_y {
            Self::Middle
        } else if pos.x < mid_x && pos.y < mid_y {
            Self::BottomLeft
        } else if pos.x > mid_x && pos.y > mid_y {
            Self::TopRight
        } else if pos.x < mid_x && pos.y > mid_y {
            Self::TopLeft
        } else {
            Self::BottomRight
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Robot {
    position: Pos<i64>,
    velocity: Pos<i64>,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_whitespace().filter_map(|x| x.get(2..)).collect();
        let position: Vec<_> = splits[0].split(",").filter_map(|x| i64::from_str(x).ok()).collect();
        let velocity: Vec<_> = splits[1].split(",").filter_map(|x| i64::from_str(x).ok()).collect();
        Ok(Self {
            position: Pos::new(position[0],position[1]),
            velocity: Pos::new(velocity[0],velocity[1]),
        })
    }
}

impl Robot {
    pub fn quadrant(&self, area: &Area<i64>, seconds: u32) -> Quadrant {
        let mid = Pos::new(area.cols() / 2 - 1, area.rows() / 2 - 1);
        let dest = self.wait(area, seconds);
        Quadrant::from_pos(&dest.position, mid.x, mid.y)
    }

    #[inline]
    pub fn wait_once(&self, area: &Area<i64>) -> Self {
        self.new_pos(area)
    }

    #[inline]
    fn wait(&self, area: &Area<i64>, seconds: u32) -> Self {
        (0..seconds).fold(*self, |r, _| r.new_pos(area))
    }

    #[inline]
    fn new_pos(&self, area: &Area<i64>) -> Self {
        Self {
            position: area.wrap(&(self.position + self.velocity)),
            velocity: self.velocity,
        }
    }
}

pub fn find_tree(robots: &[Robot], area: &Area<i64>) -> usize {
    let mut result = 1;
    let mut current: Vec<_> = robots.iter().map(|x| x.wait_once(area)).collect();
    while !has_tree(&current) && result < MAX_ITER {
        current = current.iter().map(|x| x.wait_once(area)).collect();
        result += 1;
    }
    result
}

fn has_tree(robots: &[Robot]) -> bool {
    let set: HashSet<_> = robots.iter().map(|x| x.position).collect();
    for item in &set {
        let stick = [
            item.dest(1, Direction::Down),
            item.dest(2, Direction::Down),
            item.dest(3, Direction::Down),
            item.dest(4, Direction::Down),
            item.dest(5, Direction::Down),
            item.dest(6, Direction::Down),
            item.dest(7, Direction::Down),
            item.dest(8, Direction::Down),
            item.dest(9, Direction::Down),
            item.dest(10, Direction::Down),
        ];
        if stick.iter().all(|x| set.contains(x)) {
            return true;
        }
    }
    false
}
