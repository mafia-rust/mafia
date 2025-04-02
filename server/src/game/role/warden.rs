use serde::Serialize;
use crate::game::ability_input::{AvailableBooleanSelection, AvailablePlayerListSelection};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::{game::attack_power::AttackPower, vec_set::VecSet};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;
use crate::game::Game;
use super::{
    common_role, BooleanSelection, ControllerID, ControllerParametersMap, PlayerListSelection, Role, RoleStateImpl
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
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {

        match priority {
            OnMidnightPriority::Ward => {
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
            OnMidnightPriority::Roleblock => {
                if game.day_number() == 1 {return}
                for &player in self.players_in_prison.iter() {
                    if player != actor_ref {
                        player.roleblock(game, true);
                    }
                }
            }
            OnMidnightPriority::Kill => {
                if game.day_number() == 1 {return}
                for player in self.players_to_kill(game, actor_ref) {
                    if player == actor_ref {continue}
        
                    player.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Warden),
                        AttackPower::ArmorPiercing,
                        true,
                        false
                    );
                }
            },
            _ => {}
        }

        
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> std::collections::HashSet<crate::game::chat::ChatGroup> {
        common_role::get_current_receive_chat_groups(game, actor_ref)
            .into_iter()
            .chain([crate::game::chat::ChatGroup::Warden])
            .collect()
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::combine([
                // Put players in prison
                ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Warden, 0))
                    .available_selection(AvailablePlayerListSelection {
                        available_players: PlayerReference::all_players(game)
                            .filter(|&p| p.alive(game))
                            .collect::<VecSet<_>>(),
                        can_choose_duplicates: false,
                        max_players: Some(MAX_PLAYERS_IN_PRISON)
                    })
                    .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game) || game.day_number() <= 1)
                    .reset_on_phase_start(PhaseType::Night)
                    .allow_players([actor_ref])
                    .build_map(),
                // Ward
                ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Warden, 1))
                    .available_selection(AvailableBooleanSelection)
                    .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
                    .reset_on_phase_start(PhaseType::Obituary)
                    .allow_players([actor_ref])
                    .build_map(),
            ]),
            ControllerParametersMap::combine(
                self.players_in_prison.iter().map(|&player|
                    // Live or die
                    ControllerParametersMap::builder(game)
                        .id(ControllerID::WardenLiveOrDie{warden: actor_ref, player})
                        .available_selection(AvailableBooleanSelection)
                        .allow_players([player])
                        .build_map()
                )
            )
        ])
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                let Some(PlayerListSelection(players_in_prison)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Warden, 0)
                ) else {return};

                if actor_ref.ability_deactivated_from_death(game) || players_in_prison.iter().any(|p|!p.alive(game)) {return};
                
                self.players_in_prison.clone_from(&players_in_prison);
                
                actor_ref.set_role_state(game, self);

                game.add_message_to_chat_group(
                    crate::game::chat::ChatGroup::Warden,
                    ChatMessageVariant::WardenPlayersImprisoned{
                        players: players_in_prison.to_vec()
                    }
                );
                for &player in players_in_prison.iter(){
                    InsiderGroupID::send_message_in_available_insider_chat_or_private(
                        game,
                        player,
                        ChatMessageVariant::WardenPlayersImprisoned{
                            players: players_in_prison.to_vec()
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
        if players_who_chose_die.is_empty(){
            self.players_in_prison.clone().into_iter().collect()
        }else{
            players_who_chose_die
        }
    }
}
