/// A carnival game with ticket cost, attempts, and prize.
pub struct Game {
    pub name: &'static str,
    pub tickets: u32,
    pub tries: u32,
    pub prize: &'static str,
}

const DEFAULT_TICKETS_TO_PLAY: u32 = 3;

impl Game {
    pub fn name_matches(&self, other: &str) -> bool {
        self.name.as_bytes() == other.as_bytes()
    }
}

/// Static list of carnival games.
pub fn get_games() -> &'static [Game] {
    &[
        Game { name: "Ring Toss", tickets: DEFAULT_TICKETS_TO_PLAY, tries: 5, prize: "teddy bear" },
        Game { name: "I Got It!", tickets: DEFAULT_TICKETS_TO_PLAY, tries: 12, prize: "goldfish" },
        Game { name: "Ladder Climb", tickets: DEFAULT_TICKETS_TO_PLAY, tries: 1, prize: "popcorn bucket" },
    ]
}
