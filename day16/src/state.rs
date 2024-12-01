use std::cmp::Ordering;
use std::collections::HashSet;
use aocrs::geo::direction::DirectionalPos;
use aocrs::geo::pos::PosIdx;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub dpos: DirectionalPos<usize>,
    pub path: HashSet<PosIdx>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.cost.cmp(&other.cost);
        if ord == Ordering::Equal {
            self.dpos.cmp(&other.dpos)
        } else {
            ord
        }
    }
}

impl PartialOrd for State {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn new(cost: usize, dpos: DirectionalPos<usize>, path: HashSet<PosIdx>) -> Self {
        let mut path = path;
        path.insert(dpos.pos);
        Self { cost, dpos, path }
    }

    #[inline]
    pub fn neighbours(&self) -> [State; 3] {
        [
            Self::new(self.cost + 1, self.dpos.next(1), self.path.clone()),
            Self::new(self.cost + 1000, self.dpos.update_direction(self.dpos.direction.left()), self.path.clone()),
            Self::new(self.cost + 1000, self.dpos.update_direction(self.dpos.direction.right()), self.path.clone()),
        ]
    }
}
