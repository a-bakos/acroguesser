#[derive(Debug)]
pub enum Points {
    Round5,
    Round4,
    Round3,
    Round2,
    Round1,
    NoPoint,
}

impl Points {
    pub fn get_points_value(points: Points) -> u8 {
        match points {
            Points::Round5 => 1,
            Points::Round4 => 2,
            Points::Round3 => 3,
            Points::Round2 => 4,
            Points::Round1 => 5,
            Points::NoPoint => 0,
        }
    }
}
