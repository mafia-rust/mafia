use super::builder::ControllerParametersBuilder;
use serde::{Deserialize, Serialize};

use crate::vec_map::{vec_map, VecMap};

use super::super::controller_id::ControllerID;

use super::controller_parameters::*;



#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerParametersMap{
    controllers: VecMap<ControllerID, ControllerParameters>
}
impl ControllerParametersMap{
    pub fn new(abilities: VecMap<ControllerID, ControllerParameters>)->Self{
        Self{controllers: abilities}
    }
    pub fn new_controller(id: ControllerID, ability_data: ControllerParameters)->Self{
        Self{
            controllers: vec_map!((id, ability_data))
        }
    }
    pub fn builder() -> ControllerParametersBuilder {
        ControllerParametersBuilder::new()
    }
    pub fn insert_ability(&mut self, id: ControllerID, ability_data: ControllerParameters){
        self.controllers.insert(id, ability_data);
    }
    pub fn combine_overwrite(&mut self, other: Self){
        for (ability_id, ability_selection) in other.controllers {
            self.controllers.insert(ability_id, ability_selection);
        }
    }
    pub fn combine(maps: impl IntoIterator<Item=ControllerParametersMap>) -> Self {
        let mut curr = ControllerParametersMap::new(VecMap::new());

        for map in maps {
            curr.combine_overwrite(map);
        }
        
        curr
    }
    pub fn combine_overwrite_owned(self, other: Self)->Self{
        let mut out = self;
        out.combine_overwrite(other);
        out
    }
    pub fn controller_parameters(&self)->&VecMap<ControllerID, ControllerParameters>{
        &self.controllers
    }
}