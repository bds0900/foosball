use serde::Deserialize;
use chrono::{DateTime,Local,Utc, Duration, Date};
use tokio_postgres::{NoTls, Error};
use crate::{report::{Report,TeamReport}, game::MatchResult};
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

#[derive(Clone, Debug, Deserialize)]
pub struct Team{
    team_id:i32,
    goalee:i32,
    striker:i32,
}
impl Team{
    async fn get_report(&self)->Result<TeamReport, Error>{
        let (client, connection) = tokio_postgres::connect("postgres://postgres@localhost:5432", NoTls).await?;
        let row = client.query_one("select id, goalee, striker from team where id=$1", &[&self.team_id]).await?;
        let team=Team{
            team_id:row.get("id"),
            goalee:row.get("goalee"),
            striker:row.get("striker")
        };
        let rows = client.query("select * from result",&[]).await?;
        let mut test:Vec<MatchResult>=Vec::new();
        for row in rows{
            test.push(MatchResult{
                result_id:row.get("result_id"),
                game:row.get("game"),
                winner:Some(row.get("winner")),
                winner_score:row.get("winner_score"),
                loser_score:row.get("loser_score")
            })
        }

        Ok(
            TeamReport{
                report_id:1,
                team:team,
                results:test,
                avg_rate:1,
                h2h_rate:2
            }
        )
    }
}