use super::Create;

use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use chrono_tz::Australia::Sydney;

#[derive(Debug, Deserialize, Serialize)]
pub struct PointLog {
    #[serde(with = "json_time")]
    pub logged_at: DateTime<Utc>,
    pub base: isize,
    pub admin: String,
    pub team_id: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departed: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trivia: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clues: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl PointLog {
    pub fn new(logged_at: DateTime<Utc>, team_id: isize, base: isize, admin: &str) -> PointLog {
        PointLog {
        logged_at: logged_at,
        team_id: team_id,
        base: base,
        admin: String::from(admin),
        arrived: None,
        departed: None,
        points: None,
        trivia: None,
        clues: None,
        comment: None,
        }
    }
}

impl Clone for PointLog {
    fn clone(&self) -> Self {
        PointLog {
            logged_at: self.logged_at.clone(),
            team_id: self.team_id.clone(),
            base: self.base.clone(),
            admin: self.admin.clone(),
            arrived: self.arrived.clone(),
            departed: self.departed.clone(),
            points: self.points.clone(),
            trivia: self.trivia.clone(),
            clues: self.clues.clone(),
            comment: self.comment.clone(),
        }
    }
}

impl Create for PointLog {
    fn create(&self) -> Self {
        self.clone()
    }
}

pub fn time_to_json(t: DateTime<Utc>) -> String {
	t.with_timezone(&Sydney).to_rfc3339()
}

mod json_time {
	use super::*;
	use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};

	pub fn serialize<S: Serializer>(time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
		time_to_json(time.clone()).serialize(serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error> {
		let time: String = Deserialize::deserialize(deserializer)?;
		Ok(DateTime::parse_from_rfc3339(&time).map_err(D::Error::custom)?.with_timezone(&Utc))
	}
}