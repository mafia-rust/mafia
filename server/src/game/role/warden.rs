use serde::Serialize;
use crate::game::components::insider_group::InsiderGroupID;
use crate::{game::attack_power::AttackPower, vec_set::VecSet};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::vec_set;
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;
use crate::game::Game;
use super::{
    common_role, AbilitySelection, AvailableAbilitySelection, BooleanSelection, ControllerID, ControllerParametersMap, PlayerListSelection, Priority, Role, RoleStateImpl
};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Warden{
    // vec because order matters
    // index + 1 == role controller id
    pub players_in_prison: Vec<PlayerReference>,
}

const MAX_PLAYERS_IN_PRISON: u8 = 3;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Warden {
    type ClientRoleState = Warden;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match priority {
            Priority::Ward => {
                if let Some(BooleanSelection(chose_to_ward)) = game.saved_controllers
                    .get_controller_current_selection_boolean(
                        ControllerID::role(actor_ref, Role::Warden, 1)
                    )
                {
                    if chose_to_ward {
                        actor_ref.ward(game);
                    }
                }
            }
            Priority::Roleblock => {
                if !game.attack_convert_abilities_enabled() {return}
                for &player in self.players_in_prison.iter() {
                    if player != actor_ref {
                        player.roleblock(game, true);
                    }
                }
            }
            Priority::Kill => {
                if !game.attack_convert_abilities_enabled() {return}
                for player in self.players_to_kill(game, actor_ref) {
                    if player == actor_ref {continue}
        
                    player.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Warden),
                        AttackPower::ArmorPiercing,
                        true
                    );
                }
            },
            _ => {}
        }

        
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> std::collections::HashSet<crate::game::chat::ChatGroup> {
        common_role::get_current_receive_chat_groups(game, actor_ref)
            .into_iter()
            .chain(vec![crate::game::chat::ChatGroup::Warden].into_iter())
            .collect()
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
            super::AvailableAbilitySelection::new_player_list(
                available_players,
                false,
                Some(MAX_PLAYERS_IN_PRISON),
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game) || !game.attack_convert_abilities_enabled(),
            Some(crate::game::phase::PhaseType::Night),
            false,
            vec_set!(actor_ref)
        );

        out.combine_overwrite(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Warden, 1),
                super::AvailableAbilitySelection::new_boolean(),
                AbilitySelection::new_boolean(false),
                actor_ref.ability_deactivated_from_death(game),
                Some(crate::game::phase::PhaseType::Obituary),
                false,
                vec_set!(actor_ref)
            )
        );

        for &player in self.players_in_prison.iter() {
            out.combine_overwrite(ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::WardenLiveOrDie{warden: actor_ref, player},
                AvailableAbilitySelection::new_boolean(),
                AbilitySelection::new_boolean(true),
                false,
                None,
                false,
                vec_set!(player)
            ));
        }

        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                let Some(PlayerListSelection(players_in_prison)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Warden, 0)
                ) else {return};

                if actor_ref.ability_deactivated_from_death(game) || players_in_prison.iter().any(|p|!p.alive(game)) {return};
                
                self.players_in_prison = players_in_prison.clone();
                
                actor_ref.set_role_state(game, self);

                game.add_message_to_chat_group(
                    crate::game::chat::ChatGroup::Warden,
                    ChatMessageVariant::WardenPlayersImprisoned{
                        players: players_in_prison.iter().cloned().collect()
                    }
                );
                for &player in players_in_prison.iter(){
                    InsiderGroupID::send_message_in_available_insider_chat_or_private(
                        game,
                        player,
                        ChatMessageVariant::WardenPlayersImprisoned{
                            players: players_in_prison.iter().cloned().collect()
                        },
                        false
                    );
                }
            },
            PhaseType::Obituary => {
                self.players_in_prison = Vec::new();
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}

impl Warden {
    fn players_to_kill(&self, game: &Game, actor_ref: PlayerReference)->VecSet<PlayerReference>{
        let mut players_who_chose_die = VecSet::new();

        for &player in self.players_in_prison.iter(){
            if
                !if let Some(BooleanSelection(chose_to_live)) = game.saved_controllers.get_controller_current_selection_boolean(
                    ControllerID::WardenLiveOrDie { warden: actor_ref, player }
                ) {
                    chose_to_live
                } else {true}
            {
                players_who_chose_die.insert(player);
            }
        }
        if players_who_chose_die.len() == 0{
            self.players_in_prison.clone().into_iter().collect()
        }else{
            players_who_chose_die
        }
    }
}
