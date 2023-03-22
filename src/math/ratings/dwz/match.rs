use super::player::DWZPlayer;

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
    /// use math::ratings::dwz::{player::DWZPlayer, match::Match};
    /// 
    /// let beth = DWZPlayer::new((1193.0, 1), 18);
    /// let eric = DWZPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&beth, &eric, 255) // 255 is used here for not yet decided
    /// ```
    pub fn new<'a>(player_one: &'a mut DWZPlayer, player_two: &'a mut DWZPlayer, result: f32) -> Match<'a> {
        Match { player_one, player_two, result }
    }
    /// Update the ratings of both players.
    /// # Examples
    /// ```
    /// use math::ratings::dwz::{player::DWZPlayer, match::Match};
    /// 
    /// let beth = DWZPlayer::new((1193.0, 1), 18);
    /// let eric = DWZPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&beth, &eric, 255) // 255 is used here for not yet decided
    /// first_match.result = 1.0; // Beth wins
    /// first_match.update_ratings();
    /// 
    /// assert_eq!(1261.0, beth.dwz.0.round());
    /// assert_eq!(1261.0, eric.dwz.0.round());
    /// ```
    pub fn update_ratings(&mut self) {
        let p2_res = match self.result {
            x if x == 1.0 => { 0.0 }
            x if x == 0.5 => { 0.5 }
            x if x == 0.0 => { 1.0 }
            _             => { panic!("Unexpected value for game result: {}", self.result) }
        };
        self.player_one.update_rating(vec![self.player_two.dwz.0], self.result);
        self.player_one.update_rating(vec![self.player_one.dwz.0], p2_res);
    }
}