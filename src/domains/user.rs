use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize, Display, Clone)]
pub struct UserID(pub Uuid);

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize, Display, Clone)]
pub struct Username(pub String);

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize, Display, Clone)]
pub struct Password(pub String);

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize, Display, Clone)]
pub struct Email(pub String);
