use std::cmp::min;

use crate::math::general::NumTools;

pub struct DWZPlayer {
    pub dwz:             (f32, u16),
    pub age:             u8,
}

/// ```
/// use lib_rapid::math::ratings::dwz::player::DWZPlayer;
/// 
/// let mut player = DWZPlayer::new((1193.0, 1), 18);
/// player.update_rating(vec![1213.0], 1.0);
/// 
/// assert_eq!(1261.0, player.dwz.0.round());
/// ```
impl DWZPlayer {
    pub fn new(dwz: (f32, u16), age: u8) -> DWZPlayer {
        DWZPlayer { dwz, age }
    }

    pub fn update_rating(&mut self, opponent_ratings: Vec<f32>, points: f32) {
        let mut j: f32 = 5.0;
        if self.age.is_in_range(21, 25)
        { j = 10.0; }
        else if self.age > 25
        { j = 15.0; }

        let mut expected: Vec<f32> = Vec::with_capacity(opponent_ratings.len());

        for r in &opponent_ratings {
            expected.push((1.0+10.0_f32.powf((*r - self.dwz.0) / 400.0)).recip())
        }
        let total_expected: f32 = expected.iter().sum();
        let mut accel_factor: f32 = match self.age < 20 && points > total_expected {
            true  => { self.dwz.0 / 2000.0 }
            false => { 1.0 }
        };
        if accel_factor < 0.5
        { accel_factor = 0.5; }
        else if accel_factor > 1.0
        { accel_factor = 1.0; }

        let e0 = (self.dwz.0 / 1000.0).powi(4) + j;
        let braking_val = f32::exp((1300.0 - self.dwz.0) / 150.0) - 1.0;

        let mut e = accel_factor * e0 + braking_val;

        let _temp = match braking_val == 0.0 {
            true  => { min(30, 5 * self.dwz.1) }
            false => { 150 }
        };
        if self.dwz.1 > 0 {
            if e < 5.0
            { e = 5.0; }
            else if braking_val == 0.0
            { e = _temp as f32; }
        }
        
        self.dwz.0 = self.dwz.0 + ((800.0 / (e + opponent_ratings.len() as f32) * (points - total_expected)));
        self.dwz.1.inc();
    }
}