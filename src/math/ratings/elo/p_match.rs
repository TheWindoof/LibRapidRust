use super::player::EloPlayer;

/// A match between two opponents.
pub struct Match<'a> {
    player_one: &'a mut EloPlayer,
    player_two: &'a mut EloPlayer,
    /// 1: Player one wins. 0: Player two wins.
    pub result:         f32
}

impl Match<'_> {
    /// Creates a new match between two players.
    /// # Arguments
    /// * `player_one: &'a mut EloPlayer` - The first player.
    /// * `player_two: &'a mut EloPlayer` - The second player.
    /// * `result: f32` - The result. 1: Player one wins. 0: Player two wins.
    /// # Returns
    /// A new `Match<'a>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::elo::{player::EloPlayer, p_match::Match};
    /// 
    /// let mut beth = EloPlayer::new((1193.0, 1), 18);
    /// let mut eric = EloPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&mut beth, &mut eric, -1.0); // -1.0 is used here for not yet decided
    /// ```
    pub fn new<'a>(player_one: &'a mut EloPlayer, player_two: &'a mut EloPlayer, result: f32) -> Match<'a> {
        Match { player_one, player_two, result }
    }
    /// Update the ratings of both players.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::elo::{player::EloPlayer, p_match::Match};
    /// 
    /// let mut beth = EloPlayer::new((1193.0, 1), 18);
    /// let mut eric = EloPlayer::new((1213.0, 1), 25);
    /// 
    /// let mut first_match = Match::new(&mut beth, &mut eric, -1.0); // -1.0 is used here for not yet decided
    /// first_match.result = 1.0; // Beth wins
    /// first_match.update_ratings();
    /// 
    /// assert_eq!(1214.0, beth.elo.0.round());
    /// assert_eq!(1192.0, eric.elo.0.round());
    /// ```
    pub fn update_ratings(&mut self) {
        let p2_res = match self.result {
            x if x == 1.0 => { 0.0 }
            x if x == 0.5 => { 0.5 }
            x if x == 0.0 => { 1.0 }
            _             => { panic!("Unexpected value for game result: {}", self.result) }
        };
        let temp_elo_1 = self.player_one.elo.0;
        let temp_elo_2 = self.player_two.elo.0;
        self.player_one.update_rating(temp_elo_2, self.result);
        self.player_two.update_rating(temp_elo_1, p2_res);
    }
}