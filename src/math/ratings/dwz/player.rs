use std::cmp::min;

use crate::math::general::NumTools;

/// The player necessary for a DWZ evaluation.
pub struct DWZPlayer {
    pub dwz:         (f32, u16),
    pub age:         u8,
    dev_coefficient: f32
}

impl DWZPlayer {
    /// Create a new player.
    /// # Arguments
    /// * `dwz: (f32, u16)` - The rating. `f32` is the actual rating, `u16` the index which indicates the played tournaments.
    /// * `age: u8` - The age of the player.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::dwz::player::DWZPlayer;
    /// 
    /// let mut player = DWZPlayer::new((1193.0, 1), 18);
    /// ```
    pub fn new(dwz: (f32, u16), age: u8) -> DWZPlayer {
        DWZPlayer { dwz, age, dev_coefficient: 0.0 }
    }
    /// Update a player's rating.
    /// # Arguments
    /// * `opponent_ratings: Vec<f32>` - All the opponent's ratings without the index.
    /// * `scored: f32` - The scored points. 1 is a win, 0.5 a draw and 0 a loss.
    /// * `age_coefficients: &[f32; 3]` - Three numbers corresponding to teenagers (0 - 20), young adults (21 - 25) and adults (age > 25). The higher the value, the lower the change in rating per loss or win. Use `math::ratings::dwz::player::STD_AGE_COEFFICIENTS` for the standard values.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::dwz::player::{DWZPlayer, STD_AGE_COEFFICIENTS};
    /// 
    /// let mut player = DWZPlayer::new((1193.0, 1), 18);
    /// player.update_rating(vec![1213.0], 1.0, &STD_AGE_COEFFICIENTS);
    /// 
    /// assert_eq!(1261.0, player.dwz.0.round());
    /// ```
    pub fn update_rating(&mut self, opponent_ratings: Vec<f32>, scored: f32, age_coefficients: &[f32; 3]) {
        let mut expected: Vec<f32> = Vec::with_capacity(opponent_ratings.len());

        for r in &opponent_ratings {
            expected.push((1.0+10.0_f32.powf((*r - self.dwz.0) / 400.0)).recip())
        }
        let total_expected: f32 = expected.iter().sum();
        self.update_dev_coefficient(scored, total_expected, age_coefficients);
        
        self.dwz.0 = self.dwz.0 + ((800.0 / (self.dev_coefficient + opponent_ratings.len() as f32) * (scored - total_expected)));
        self.dwz.1.inc();
    }

    fn update_dev_coefficient(&mut self, scored: f32, expected: f32, age_coefficients: &[f32; 3]) {
        let mut j: f32 = age_coefficients[0];
        if self.age.is_in_range(21, 25)
        { j = age_coefficients[1]; }
        else if self.age > 25
        { j = age_coefficients[2]; }

        let mut accel_factor: f32 = match self.age < 20 && scored > expected {
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

        self.dev_coefficient = e;
    }
}
/// The standard age coefficients of the DWZ rating system.
/// * Teenagers between 0 and 20 years of age: 5.
/// * Junior Adults between 21 and 25 years: 10.
/// * Adults over 25: 15.
pub const STD_AGE_COEFFICIENTS: [f32; 3] = [ // Teenagers (0 - 20 yrs.)
                                        5.0,
                                        // Junior Adults (21 - 25 yrs.)
                                        10.0,
                                        // Adults ( age > 25 yrs.)
                                        15.0
                                    ];