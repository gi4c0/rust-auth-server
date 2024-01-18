use derive_more::AsRef;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize)]
pub struct UserID(pub String);

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize)]
pub struct Username(pub String);

#[derive(PartialEq, AsRef, Debug, Deserialize, Serialize)]
pub struct Email(pub String);
