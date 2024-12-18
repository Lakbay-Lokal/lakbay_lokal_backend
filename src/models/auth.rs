use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserClaims {
    pub user_id: i32,
    pub exp: i64,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BusClaims {
    pub bus_id: i32,
    pub exp: i64,
}
