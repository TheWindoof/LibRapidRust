use crate::math::general::NumTools;

/// The player necessary for a Elo evaluation.
pub struct EloPlayer {
    /// Actual Rating and played matches.
    pub elo:  (f32, u16),
    pub age:  u8,
    /// The development factor called 'k'. The higher the factor, the quicker the rating changes.
    k_factor: u8
}

impl EloPlayer {
    /// Create a new player.
    /// # Arguments
    /// * `elo: (f32, u16)` - The rating. `f32` is the actual rating, `u16` the index which indicates the played matches.
    /// * `age: u8` - The age of the player.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::elo::player::EloPlayer;
    /// 
    /// let mut player = EloPlayer::new((2306.0, 32), 43);
    /// ```
    pub fn new(elo: (f32, u16), age: u8) -> EloPlayer {
        let mut res = EloPlayer { elo, age, k_factor: 0 };
        res.update_k_factor();
        res
    }
    /// Update a player's rating.
    /// # Arguments
    /// * `opponent_ratings: Vec<f32>` - All the opponent's ratings without the index.
    /// * `scored: f32` - The scored points. 1 is a win, 0.5 a draw and 0 a loss.
    /// # Examples
    /// ```
    /// use lib_rapid::math::ratings::elo::player::EloPlayer;
    /// 
    /// let mut player = EloPlayer::new((2306.0, 32), 43);
    /// player.update_rating(2077.0, 0.0);
    /// 
    /// assert_eq!(2290.0, player.elo.0.round());
    /// ```
    pub fn update_rating(&mut self, opponent_rating: f32, scored: f32) {
        self.update_k_factor();

        let expected = (1.0+10.0_f32.powf((opponent_rating - self.elo.0) / 400.0)).recip();
        
        self.elo.0 = self.elo.0 + self.k_factor as f32 * (scored - expected);
        self.elo.1.inc();
    }

    fn update_k_factor(&mut self) {
        if (self.age >= 18 && self.elo.0 < 2300.0) || self.elo.1 < 30
        { self.k_factor = 40; }
        else if self.elo.0 < 2400.0 && self.elo.1 >= 30
        { self.k_factor = 20; }
        else
        { self.k_factor = 10; }
    }
}