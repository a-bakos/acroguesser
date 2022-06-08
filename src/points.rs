const MAX_POINTS: u8 = 10;
const MED_POINTS: u8 = 5;
const MIN_POINTS: u8 = 1;

#[derive(Debug)]
pub enum Points {
    Max,
    Med,
    Min,
}

impl Points {
    pub fn add_points_value(points: Points) -> u8 {
        match points {
            Points::Max => MAX_POINTS,
            Points::Med => MED_POINTS,
            Points::Min => MIN_POINTS,
        }
    }
}
