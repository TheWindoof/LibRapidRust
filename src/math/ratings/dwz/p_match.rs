use super::player::DWZPlayer;

/// A match between two opponents.
pub struct Match<'a> {
    player_one: &'a mut DWZPlayer,
    player_two: &'a mut DWZPlayer,
    /// 1: Player one wins. 0: Player two wins.
    pub result:         f32
}

impl Match<'_> {
    /// Creates a new match between two players.
    /// # Arguments
    /// * `player_one: &'a mut DWZPlayer` - The first player.
    /// * `player_two: &'a mut DWZPlayer` - The second player.
    /// * `result: f32` - The result. 1: Player one wins. 0: Player two wins.
    /// # Returns
    /// A new `Match<'a>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::dwz::{player::DWZPlayer, p_match::Match};
    /// 
    /// let mut beth = DWZPlayer::new((1193.0, 1), 18);
    /// let mut eric = DWZPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&mut beth, &mut eric, -1.0); // -1.0 is used here for not yet decided
    /// ```
    pub fn new<'a>(player_one: &'a mut DWZPlayer, player_two: &'a mut DWZPlayer, result: f32) -> Match<'a> {
        Match { player_one, player_two, result }
    }
    /// Update the ratings of both players.
    /// # Arguments
    /// * `age_coefficients: &[f32; 3]` - Three numbers corresponding to teenagers (0 - 20), young adults (21 - 25) and adults (age > 25). The higher the value, the lower the change in rating per loss or win. Use `math::ratings::dwz::player::std_age_coefficients` for the standard values.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::dwz::{player::{DWZPlayer, STD_AGE_COEFFICIENTS}, p_match::Match};
    /// 
    /// let mut beth = DWZPlayer::new((1193.0, 1), 18);
    /// let mut eric = DWZPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&mut beth, &mut eric, -1.0); // -1.0 is used here for not yet decided
    /// first_match.result = 1.0; // Beth wins
    /// first_match.update_ratings(&STD_AGE_COEFFICIENTS);
    /// 
    /// assert_eq!(1261.0, beth.dwz.0.round());
    /// assert_eq!(1183.0, eric.dwz.0.round());
    /// ```
    pub fn update_ratings(&mut self, age_coefficients: &[f32; 3]) {
        let p2_res = match self.result {
            x if x == 1.0 => { 0.0 }
            x if x == 0.5 => { 0.5 }
            x if x == 0.0 => { 1.0 }
            _             => { panic!("Unexpected value for game result: {}", self.result) }
        };
        let temp_dwz_1 = self.player_one.dwz.0;
        let temp_dwz_2 = self.player_two.dwz.0;
        self.player_one.update_rating(vec![temp_dwz_2], self.result, age_coefficients);
        self.player_two.update_rating(vec![temp_dwz_1], p2_res, age_coefficients);
    }
}