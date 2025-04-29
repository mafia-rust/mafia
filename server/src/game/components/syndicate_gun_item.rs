use crate::{game::{
    ability_input::*
}, vec_set};

use super::{detained::Detained, insider_group::InsiderGroupID, mafia::MafiaAttacker, night_visits::NightVisits, tags::{TagSetID, Tags}};

#[derive(Default)]
pub struct SyndicateGunItem {
    player_with_gun: Option<PlayerReference>
}

impl SyndicateGunItem {
    pub fn on_visit_wardblocked(_game: &mut Game, midnight_variables: &mut MidnightVariables, visit: Visit){
        NightVisits::retain(midnight_variables, |v|
            v.tag != VisitTag::SyndicateGunItem || v.visitor != visit.visitor
        );
    }
    pub fn on_player_roleblocked(_game: &mut Game, midnight_variables: &mut MidnightVariables, player: PlayerReference){
        NightVisits::retain(midnight_variables, |v|
            v.tag != VisitTag::SyndicateGunItem || v.visitor != player
        );
    }

    pub fn give_gun_to_player(game: &mut Game, player: PlayerReference) {
        Self::remove_gun(game);
        game.syndicate_gun_item.player_with_gun = Some(player);
        
        Tags::add_tag(game, TagSetID::SyndicateGun, player);
    }
    /// Returns which player is given the gun if any.
    pub fn give_gun_to_insider(game: &mut Game) -> Option<PlayerReference> {
        if game.syndicate_gun_item.player_with_gun.is_some_and(|p|
            p.alive(game) && 
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, p)
        ) {return None}
        game.syndicate_gun_item.player_with_gun = None;

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            Tags::remove_tag(game, TagSetID::SyndicateGun, insider);
        }
        let insider = InsiderGroupID::Mafia.players(game)
            .iter()
            .find(|p|p.alive(game))
            .copied()?;
        SyndicateGunItem::give_gun_to_player(game, insider);
        Some(insider)
    } 
    pub fn remove_gun(game: &mut Game) {
        game.syndicate_gun_item.player_with_gun = None;

        Tags::set_tagged(game, super::tags::TagSetID::SyndicateGun, &vec_set![]);
    }

    pub fn player_with_gun(&self) -> Option<PlayerReference> {
        self.player_with_gun
    }
    pub fn player_has_gun(game: &Game, player: PlayerReference) -> bool{
        game.syndicate_gun_item.player_with_gun().is_some_and(|s|s==player)
    }

    //available ability
    pub fn controller_parameters_map(game: &Game) -> ControllerParametersMap {
        if let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun {
            ControllerParametersMap::combine([
                ControllerParametersMap::builder(game)
                    .id(ControllerID::syndicate_gun_item_shoot())
                    .single_player_selection_typical(player_with_gun, false, false)
                    .night_typical(player_with_gun)
                    .add_grayed_out_condition(game.day_number() <= 1)
                    .build_map(),
                ControllerParametersMap::builder(game)
                    .id(ControllerID::syndicate_gun_item_give())
                    .available_selection(AvailablePlayerListSelection {
                        available_players: PlayerReference::all_players(game)
                            .filter(|target|
                                player_with_gun != *target &&
                                target.alive(game) &&
                                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target))
                            .collect(),
                        can_choose_duplicates: false,
                        max_players: Some(1)
                    })
                    .add_grayed_out_condition(
                        Detained::is_detained(game, player_with_gun) ||
                        !player_with_gun.alive(game)
                    )
                    .reset_on_phase_start(PhaseType::Obituary)
                    .dont_save()
                    .allow_players([player_with_gun])
                    .build_map()
            ])
        }else{
            ControllerParametersMap::default()
        }
    }

    //event listeners
    pub fn on_add_insider(game: &mut Game, event: &OnAddInsider, _fold: &mut (), _priority: ()){
        if event.group == InsiderGroupID::Mafia {
            Tags::set_viewers(game, TagSetID::SyndicateGun, &InsiderGroupID::Mafia.players(game).clone());
        }
    }
    pub fn on_remove_insider(game: &mut Game, event: &OnRemoveInsider, _fold: &mut (), _priority: ()){
        if event.group == InsiderGroupID::Mafia {
            Tags::set_viewers(game, TagSetID::SyndicateGun, &InsiderGroupID::Mafia.players(game).clone());
            if game.syndicate_gun_item.player_with_gun == Some(event.player) {
                MafiaAttacker::Gun.on_removal(game, event.player);
            }
        }
    }
    
    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        if game.syndicate_gun_item.player_with_gun == Some(player) {
            MafiaAttacker::Gun.on_removal(game, player);
        }
    }


    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority) {
        if game.day_number() <= 1 {return}
        match priority {
            OnMidnightPriority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 

                let Some(PlayerListSelection(gun_target)) = ControllerID::syndicate_gun_item_shoot().get_player_list_selection(game) else {return};
                let Some(gun_target) = gun_target.first() else {return};

                NightVisits::add_visit(
                    midnight_variables, 
                    Visit::new(player_with_gun, *gun_target, true, VisitTag::SyndicateGunItem)
                );
            }
            OnMidnightPriority::Kill => {
                let targets: Vec<(PlayerReference, PlayerReference)> = NightVisits::all_visits(midnight_variables)
                    .iter()
                    .filter(|visit| visit.tag == VisitTag::SyndicateGunItem)
                    .map(|visit| (visit.visitor, visit.target))
                    .collect();

                for (attacker, target) in targets {
                    target.try_night_kill_single_attacker(
                        attacker,
                        game, midnight_variables,
                        GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::Basic,
                        false
                    );
                }
            }
            _ => {}
        }
    }
    
    pub fn on_validated_ability_input_received(game: &mut Game, actor_ref: PlayerReference, ability_input: AbilityInput) {
        if game.syndicate_gun_item.player_with_gun.is_none_or(|p|p != actor_ref) {return}

        let Some(PlayerListSelection(target)) = ability_input
            .get_player_list_selection_if_id(ControllerID::SyndicateGunItemGive)
        else {return};
        let Some(target) = target.first() else {return};

        if actor_ref != *target &&
            target.alive(game) &&
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target) 
        {
            SyndicateGunItem::give_gun_to_player(game, *target);
        }
    }
}