use chrono::{DateTime,Local,Utc, Duration, Date};
fn main() {
    println!("Hello, world!");
}
enum Status {
    Ongoing,
    Past
}
pub struct Report{

}
pub struct TeamReport{
    report_id:i32,
    team:Team,
    results:Vec<MatchResult>,
    avg_rate:u8,
    h2h_rate:u8,
}
pub struct Player{
    player_id:i32,
    name:&'static str,
    win_rate:u32,
    start_date:DateTime<Local>
}
impl Player{
    fn get_report()->Report{
        Report{
            
        }
    }
    
}
pub struct Team{
    team_id:i32,
    goalee:Player,
    striker:Player,
}
impl Team{
    fn get_report()->TeamReport{

        TeamReport{

        }
    }
}
pub struct Match{
    match_id:i32,
    red:Team,
    blue:Team,
    date:DateTime<Local>,
    status:Status
}
pub struct MatchResult{
    result_id:i32,
    game:Match,
    winner:Option<Team>,
    winner_score:u8,
    loser_score:u8,
}
impl MatchResult{
    fn add_score(team:&str){

    }
    
}

pub struct GameManager{

}
impl GameManager{
    fn get_result(id:i32){

    }
    fn get_last_result(){

    }
    fn get_results(){

    }
    fn get_results_by_player_id(id:i32){

    }
    fn get_results_by_match_id(id:i32){

    }
    fn record_result(result:MatchResult){

    }
}



