use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Map;

#[derive(Serialize, Deserialize)]
pub enum DocumentType {
    User(UserType),
    Skill(SkillType),
    Photos(PhotoListType),
    Messages(MessageListType),
    Uu(RelationUserUserType),
    Us(RelationUserSkillType),
}

#[derive(Serialize, Deserialize)]
pub struct UserType {
    pub name: String,
    pub pseudo: String,
    pub email: String,
    pub birth_date: String,
    pub level: i32,
    pub password: String,
    pub title: String,
    pub title_list : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct RelationUserUserType {
    pub _from: String,
    pub _to: String,
    pub force: i32,
    pub time: i32,
    pub relation_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelationUserSkillType {
    pub _key: String,
    pub from: String,
    pub to: String,
    pub level: i32,
    pub exp : i32,
    pub exp_to_next : i32,
    pub challenge_completed: HashMap<i32,HashMap<i32, ChallengeType>>,
}
#[derive(Serialize, Deserialize)]
pub struct PhotoListType {
    pub _key: Option<String>,
    pub wall: Vec<PhotoType>,
    pub storie: Vec<PhotoType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PhotoType {
    pub image: String,
    pub title: String,
    pub likes: i32,
    pub comments: Vec<String>,
    pub shared: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageListType {
    pub _from: String,
    pub _to: String,
    pub messages: Vec<MessageType>,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageType {
    pub message: String,
    pub state: MessageState,
    pub date: String,
    pub from: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MessageState {
    SENDING,SENT,SEEN
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkillType {
    pub name: String,
    pub challenges: Vec<Vec<ChallengeType>>,
    pub progression_rewards : Vec<Vec<RewardType>>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RewardType {
    Title(String)
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ChallengeType {
    pub message: String,
    pub points: i32,
    pub photo: Option<PhotoType>
}