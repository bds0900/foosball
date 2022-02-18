use std::default;

use chrono::{DateTime,Local,Utc, Duration, Date};

use crate::report::{Report,TeamReport};
pub struct Player{
    player_id:i32,
    name:&'static str,
    win_rate:i32,
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
            report_id:1,
            team:default,
            results:default,
            avg_rate:default,
            h2h_rate:default

        }
    }
}