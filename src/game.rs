use chrono::{DateTime,Local,Utc, Duration, Date};

use crate::{
    report::{Report,TeamReport},
    player::{Team}
};


enum Status {
    Ongoing,
    Past
}

pub struct Match {
    match_id: i32,
    red: Team,
    blue: Team,
    date: DateTime<Local>,
    status: Status,
}
pub struct MatchResult {
    result_id: i32,
    game: Match,
    winner: Option<Team>,
    winner_score: i32,
    loser_score: i32,
}
impl MatchResult {
    fn add_score(team: &str) {}
}
