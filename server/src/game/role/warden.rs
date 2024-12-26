use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::vec_set;
use crate::{game::attack_power::DefensePower, vec_set::VecSet};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{
    AbilitySelection, AvailableAbilitySelection, BooleanSelection, ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl, TwoPlayerOptionSelection
};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Warden{
    pub players_in_prison: VecSet<PlayerReference>,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Warden {
    type ClientRoleState = Warden;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        if game.day_number() == 1 {return}

        let all_chose_live = self.players_in_prison.iter().all(|&p|Warden::chose_to_live(game, p));
        
        for &player in self.players_in_prison.iter() {
            if
                (all_chose_live) ||
                !Warden::chose_to_live(game, player)
            {
                player.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Warden),
                    AttackPower::ArmorPiercing,
                    true
                );
            }
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let available_players = PlayerReference::all_players(game)
            .into_iter()
            .filter(|&p| p.alive(game))
            .collect::<VecSet<_>>();
        
        let mut out = 
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Warden, 0),
            super::AvailableAbilitySelection::new_two_player_option(
                available_players.clone(),
                available_players,
                false,
                true
            ),
            AbilitySelection::new_two_player_option(None),
            !actor_ref.alive(game) || game.day_number() <= 1,
            Some(crate::game::phase::PhaseType::Night),
            false,
            vec_set!(actor_ref)
        );

        for &player in self.players_in_prison.iter() {
            out.combine_overwrite(ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(player, Role::Warden, 1),
                AvailableAbilitySelection::new_boolean(),
                AbilitySelection::new_boolean(false),
                false,
                None,
                false,
                vec_set!(player)
            ));
        }

        out
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        vec![]
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                let Some(TwoPlayerOptionSelection(Some((a,b)))) = game.saved_controllers.get_controller_current_selection_two_player_option(
                    ControllerID::role(actor_ref, Role::Warden, 0)
                ) else {return};

                if !actor_ref.alive(game) || !a.alive(game) || !b.alive(game) {return};
                
                let players_in_prison = vec_set!(a,b);
                self.players_in_prison = players_in_prison.clone();
                
                actor_ref.set_role_state(game, self);

                game.add_message_to_chat_group(
                    crate::game::chat::ChatGroup::Warden,
                    ChatMessageVariant::WardenPlayersImprisoned{
                        players: players_in_prison.iter().cloned().collect()
                    }
                );
                for player in players_in_prison {
                    Detained::add_detain(game, player);    
                }
            },
            PhaseType::Obituary => {
                self.players_in_prison = VecSet::new();
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}

impl Warden {
    fn chose_to_live(game: &Game, actor_ref: PlayerReference) -> bool {
        if let Some(BooleanSelection(chose_to_live)) = game.saved_controllers.get_controller_current_selection_boolean(
            ControllerID::role(actor_ref, Role::Warden, 1)
        ) {
            chose_to_live
        } else {true}
    }
}
