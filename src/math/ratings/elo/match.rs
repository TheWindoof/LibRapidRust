use super::player::EloPlayer;

pub struct Match<'a> {
    player_one: &'a mut EloPlayer,
    player_two: &'a mut EloPlayer,
    /// 1: Player one wins. 0: Player two wins.
    result:         f32
}

impl Match<'_> {
    pub fn new<'a>(player_one: &'a mut EloPlayer, player_two: &'a mut EloPlayer, result: f32) -> Match<'a> {
        Match { player_one, player_two, result }
    }

    pub fn update_ratings(&mut self) {
        let p2_res = match self.result {
            x if x == 1.0 => { 0.0 }
            x if x == 0.5 => { 0.5 }
            x if x == 0.0 => { 1.0 }
            _             => { panic!("Unexpected value for game result: {}", self.result) }
        };
        self.player_one.update_rating(self.player_two.elo.0, self.result);
        self.player_one.update_rating(self.player_one.elo.0, p2_res);
    }
}