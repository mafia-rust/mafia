use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::detained::Detained;
use crate::game::components::poison::{Poison, PoisonAlert};
use crate::game::{attack_power::DefensePower, components::puppeteer_marionette::PuppeteerMarionette};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{AbilitySelection, ControllerID, ControllerParametersMap, IntegerSelection, Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Puppeteer{
    pub marionettes_remaining: u8,
}

impl Default for Puppeteer{
    fn default() -> Self {
        Self {
            marionettes_remaining: 3,
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Puppeteer {
    type ClientRoleState = Puppeteer;
    fn new_state(game: &Game) -> Self {
        Self{
            marionettes_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Poison {return;}


        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target = visit.target;
            
            if game.saved_controllers.get_controller_current_selection_integer(
                ControllerID::role(actor_ref, Role::Puppeteer, 1)
            ).unwrap_or(IntegerSelection(0)).0 == 1 {
                if AttackPower::ArmorPiercing.can_pierce(target.defense(game)) {
                    if PuppeteerMarionette::string(game, target){
                        self.marionettes_remaining -= 1;
                    }
                    actor_ref.set_role_state(game, RoleState::Puppeteer(self));
                }else{
                    Poison::poison_player(game, 
                        target, AttackPower::ArmorPiercing, 
                        crate::game::grave::GraveKiller::Role(Role::Puppeteer), 
                        vec![actor_ref].into_iter().collect(), true,
                        PoisonAlert::Alert,
                    );
                }
            }else{
                Poison::poison_player(game, 
                    target, AttackPower::ArmorPiercing, 
                    crate::game::grave::GraveKiller::Role(Role::Puppeteer), 
                    vec![actor_ref].into_iter().collect(), true,
                    PoisonAlert::Alert,
                );
            }
        }

        
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Puppeteer, 0),
            super::AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|&p|
                        actor_ref != p &&
                        p.alive(game) &&
                        !PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&p)
                    )
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            Detained::is_detained(game, actor_ref) || !actor_ref.alive(game),
            None,
            false,
            vec_set!(actor_ref),
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Puppeteer, 1),
                super::AvailableAbilitySelection::new_integer(0, 
                    if self.marionettes_remaining > 0 {1} else {0}
                ),
                AbilitySelection::new_integer(0),
                false,
                None,
                false,
                vec_set!(actor_ref),
            )
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Puppeteer, 0),
            false,
        )
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Puppeteer
        ].into_iter().collect()
    }
}