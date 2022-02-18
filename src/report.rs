
use crate::game::{
    MatchResult
};
use crate::player::{
    Team,
    Player
};

pub struct Report{

}
pub struct TeamReport{
    report_id:i32,
    team:Team,
    results:Vec<MatchResult>,
    avg_rate:i32,
    h2h_rate:i32,
}