use serde::{Deserialize, Serialize};

use crate::game::role::Role;


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOptionSelection(pub Option<Role>);