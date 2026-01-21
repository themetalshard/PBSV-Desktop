use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ScheduleEvent {
    #[serde(rename = "Time")]
    pub time: i64,

    #[serde(rename = "Duration")]
    pub duration: i64,

    #[serde(rename = "EventType")]
    pub event_type: String,

    #[serde(rename = "Trainer")]
    pub trainer: Option<String>,

    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    #[serde(rename = "TrainerId")]
    pub trainer_id: Option<u64>,

    #[serde(rename = "TrainerCommsId")]
    pub discord_id: Option<String>,

    #[serde(rename = "TrainingID")]
    pub uuid: Option<String>,

    #[serde(rename = "EventColor")]
    pub event_color: Option<[u8; 3]>,
}
