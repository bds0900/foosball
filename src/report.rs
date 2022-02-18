
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
    pub report_id:i32,
    pub team:Team,
    pub results:Vec<MatchResult>,
    pub avg_rate:i32,
    pub h2h_rate:i32,
}