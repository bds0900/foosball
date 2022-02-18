use chrono::{DateTime,Local,Utc, Duration, Date};
use postgres::{Client, NoTls, Error};
use serde_derive::Deserialize;
use crate::{
    report::{Report,TeamReport},
    player::{Team}
};
use postgres_types::{ToSql, FromSql};
enum Status {
    Ongoing,
    Past
}

pub struct Match {
    match_id: i32,
    red: i32,
    blue: i32,
    date: DateTime<Local>,
    status: Status,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchResult {
    pub result_id: i32,
    pub game: i32,
    pub winner: Option<i32>,
    pub winner_score: i32,
    pub loser_score: i32,
}
impl MatchResult {
    fn add_score(team: &str) {}
}
