use crate::consts;

#[derive(Debug)]
pub enum Points {
    Max,
    Med,
    Min,
}

impl Points {
    pub fn add_points_value(points: Points) -> u8 {
        match points {
            Points::Max => consts::MAX_POINTS,
            Points::Med => consts::MED_POINTS,
            Points::Min => consts::MIN_POINTS,
        }
    }
}
