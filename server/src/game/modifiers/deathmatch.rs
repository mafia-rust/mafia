
use crate::game::{ability_input::{AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap}, attack_type::{AttackData, AttackType}, chat::{ChatGroup, ChatMessageVariant}, components::{mafia_recruits::MafiaRecruits, pitchfork::Pitchfork, poison::Poison, puppeteer_marionette::PuppeteerMarionette, syndicate_gun_item::SyndicateGunItem}, phase::{PhaseState, PhaseType}, player::PlayerReference, Game};
use crate::vec_set;
use super::{ModifierState, ModifierTrait, ModifierType, Modifiers};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Deathmatch{
    day_of_last_death: u8
}

/*
    There is modifier specific code in server.src\game\role\mod.rs
    in the defense function for role
*/
impl From<&Deathmatch> for ModifierType{
    fn from(_: &Deathmatch) -> Self {
        ModifierType::Deathmatch
    }
}

impl ModifierTrait for Deathmatch {
    fn on_game_start(self, game: &mut Game) {
        for player in PlayerReference::all_players(game){
            game.add_message_to_chat_group(
                ChatGroup::All, 
                ChatMessageVariant::PlayerHasWinCondition{player: player.index(), win_condition: player.win_condition(game).clone()}
            );
        }
    }
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        if phase.phase() == PhaseType::Nomination {
            game.on_fast_forward();
        }
    }
    fn on_any_death(self, game: &mut Game, _player:PlayerReference) {
        Modifiers::set_modifier(game, Deathmatch{day_of_last_death: game.day_number()}.into());
    }
    fn on_ability_input_received(self,_game: &mut Game,_actor_ref:PlayerReference,_input:crate::game::ability_input::AbilityInput) {
        
    }
}

impl Deathmatch {
    pub fn controller_parameters_map(game: &Game)-> ControllerParametersMap{
        if !Modifiers::modifier_is_enabled(game, ModifierType::Deathmatch) {
            return ControllerParametersMap::default();
        }

        let mut out = ControllerParametersMap::default();
        
        
        for player in PlayerReference::all_players(game){
            out.combine_overwrite(
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::OptForDraw{player},
                    AvailableAbilitySelection::new_boolean(),
                    AbilitySelection::new_boolean(false),
                    player.alive(game),
                    None,
                    false,
                    vec_set![player]
                )
            );
        }
        out
    }
    pub fn players_opt_for_draw(game: &Game) -> bool {
        PlayerReference::all_players(game).all(|p|
            !p.alive(game) || 
            ControllerID::OptForDraw{player: p}.get_boolean_selection(game).is_some_and(|b|b.0)
        )
    }
    pub fn is_draw(game: &Game) -> bool {
        if let Some(ModifierState::Deathmatch(deathmatch)) = Modifiers::get_modifier(game, ModifierType::Deathmatch) {
            if deathmatch.day_of_last_death.saturating_add(5) >= game.day_number() {
                return true
            }
        }

        
        if Self::players_opt_for_draw(game) {
            return true
        }
        if MafiaRecruits::any_recruits(game) {
            return false;
        }
        if Poison::any_players_poisoned(game) {
            return false
        }
        if PuppeteerMarionette::any_marionettes(game) {
            return false
        }
        if Pitchfork::any_remaining_pitchforks(game) {
            return false
        }

        let mut possessable_dead_attacker= AttackData::none();
        let mut revivable_dead_attacker= AttackData::none();
        let mut reviver= AttackData::none();
        let mut possessor= AttackData::none();
        let mut dead_wildcard: bool = false;

        for player in PlayerReference::all_players(game) {
            let data = player.role_state(game).attack_data(game, player);           
            match (&data.attack_type, player.alive(game)) {
                (AttackType::None, _) | 
                (AttackType::NecroPossess {..}, false) | (AttackType::Revive {..}, false) => (),

                (AttackType::AttackDead, _) | (AttackType::Attack{..}, true) |
                (AttackType::Wildcard, true) => return false,

                (AttackType::Attack{possess_immune: false}, false) => {
                    if possessable_dead_attacker.is_none() || possessable_dead_attacker.town_on_grave <= data.town_on_grave {
                        possessable_dead_attacker = data;
                    }
                },
                (AttackType::Attack{possess_immune: true}, false) => {
                    if revivable_dead_attacker.is_none() || revivable_dead_attacker.town_on_grave <= data.town_on_grave {
                        revivable_dead_attacker = data;
                    }
                },

                (AttackType::NecroPossess {town_only}, true) => {
                    if possessor.is_none() ||
                        if let AttackType::NecroPossess{town_only: necro_town_only } = possessor.attack_type {
                            *town_only <= necro_town_only
                        } else {
                            unreachable!()
                        }
                    {
                        possessor = data
                    }
                }
                
                (AttackType::Revive{town_only}, true) => {
                    if reviver.is_none() ||
                        if let AttackType::Revive{town_only: reviver_town_only} = reviver.attack_type {
                            *town_only <= reviver_town_only
                        } else {
                            unreachable!()
                        }
                    {
                        reviver = data
                    }
                }

                (AttackType::Wildcard, false) => {
                    dead_wildcard = true;
                }
            };
        }
        if let Some(gun) = SyndicateGunItem::player_with_gun(&game.syndicate_gun_item) {
            if gun.alive(game) {
                return false
            }
            let data = AttackData::attack(game, gun, false);
            if possessor.can_possess_to_attack(&data) {
                return false
            }
            if reviver.can_revive_to_attack(&data) {
                return false
            }
        }

        if reviver.is_revive() {
            if dead_wildcard {
                return false
            }
            if reviver.can_revive_to_attack(&possessable_dead_attacker) {
                return false
            }
            if reviver.can_revive_to_attack(&revivable_dead_attacker) {
                return false
            }
        }

        if possessor.can_possess_to_attack(&possessable_dead_attacker) {
            return false
        }

        true
    }
}