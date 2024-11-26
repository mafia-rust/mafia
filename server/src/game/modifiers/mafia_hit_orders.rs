use crate::{
    game::{
        ability_input::AbilityInput, components::insider_group::InsiderGroupID, player::PlayerReference, role::{mafioso::Mafioso, Priority}, role_list::{RoleOutline, RoleOutlineOption, RoleSet}, tag::Tag, Game
    },
    packet::ToClientPacket, vec_map::VecMap, vec_set::VecSet
};

use super::{ModifierTrait, ModifierType, Modifiers};
use vec1::vec1;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct MafiaHitOrders{
    active: bool,
    hit_order_players: VecSet<PlayerReference>,
    hit_order_vote: VecMap<PlayerReference, PlayerReference>,
}

impl From<&MafiaHitOrders> for ModifierType{
    fn from(_: &MafiaHitOrders) -> Self {
        ModifierType::MafiaHitOrders
    }
}

impl MafiaHitOrders{
    pub fn new()->Self{
        Self{
            active: true,
            hit_order_players: VecSet::new(),
            hit_order_vote: VecMap::new(),
        }
    }
    pub fn active(&self)->bool{
        self.active
    }


    pub fn is_player_marked(game: &mut Game, player: PlayerReference)->bool{
        let Some(modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return false};
        modifier.hit_order_players.contains(&player)
    }

    pub fn switch_to_mafioso_action(game: &mut Game, player: PlayerReference){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return};
        if !modifier.active {return}
        if !player.alive(game) {return}
        if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player) {return}

        //if the mafia has any mafia killing role
        if PlayerReference::all_players(game).into_iter()
            .filter(|p|InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p))
            .any(|p|RoleSet::MafiaKilling.get_roles().contains(&p.role(game)))
        {
            return;
        }

        player.set_role(game, Mafioso::default());
        player.set_selection(game, player.selection(game).clone());
        modifier.active = false;

        for insider in InsiderGroupID::Mafia.players(game).clone(){
            insider.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn mark_vote_action(game: &mut Game, voter: PlayerReference, target: Option<PlayerReference>){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return};
        if !modifier.active {return}

        if !voter.alive(game) {return}
        if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, voter) {return}
        if let Some(target) = target{
            if !target.alive(game) {return}
        }
        
        match target {
            Some(target) => modifier.hit_order_vote.insert(voter, target),
            None => modifier.hit_order_vote.remove(&voter),
        };
        voter.send_packet(game, ToClientPacket::YourHitOrderVote{player: target});
        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn remove_all_votes(game: &mut Game){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return};
        for player in modifier.hit_order_vote.keys(){
            player.send_packet(game, ToClientPacket::YourHitOrderVote{player: None});
        }
        modifier.hit_order_vote.clear();
        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn hit_order_voted_player(game: &mut Game)->Option<PlayerReference>{
        let Some(modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return None};
        let mut vote_counts = VecMap::new();
        for (_, target) in modifier.hit_order_vote.iter(){
            vote_counts.insert(
                target,
                vote_counts.get(&target).unwrap_or(&0u8)+1u8
            );
        }
        vote_counts.into_iter().max_by_key(|(_, count)|*count).map(|(player, _)|*player)
    }

    pub fn add_hit_order_player(game: &mut Game, player: PlayerReference){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return};
        if !modifier.active {return}
        if modifier.hit_order_players.insert(player).is_some() {return;}

        for insider in InsiderGroupID::Mafia.players(game).clone(){
            insider.push_player_tag(game, player, Tag::GodfatherBackup);
        }
        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn remove_hit_order_player(game: &mut Game, player: PlayerReference){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::MafiaHitOrders).cloned() else {return};
        modifier.hit_order_players.remove(&player);
        for insider in InsiderGroupID::Mafia.players(game).clone(){
            insider.remove_player_tag(game, player, Tag::GodfatherBackup);
        }
        Modifiers::set_modifier(game, modifier.into());
    }
}

impl ModifierTrait for MafiaHitOrders{
    fn on_ability_input_received(self, game: &mut Game, actor_ref:crate::game::player::PlayerReference, input:crate::game::ability_input::AbilityInput) {
        match input {
            AbilityInput::HitOrderVote { selection } => {
                MafiaHitOrders::mark_vote_action(game, actor_ref, selection.0);
            },
            AbilityInput::HitOrderMafioso => {
                MafiaHitOrders::switch_to_mafioso_action(game, actor_ref);
            }
            _ => {}
        }
    }

    fn before_phase_end(self, game: &mut Game, phase:crate::game::phase::PhaseType) {
        if !self.active {return}
        if game.day_number() == 1 {return;}
        if phase != crate::game::phase::PhaseType::Night {return}
        let Some(player) = Self::hit_order_voted_player(game) else {return};
        MafiaHitOrders::add_hit_order_player(game, player);
        MafiaHitOrders::remove_all_votes(game);
    }
    fn on_night_priority(self, game: &mut Game, priority: Priority){
        if !self.active {return}
        if game.day_number() == 1 {return;}

        match priority {
            //set visits to attacking visits
            Priority::Deception => {
                for player in PlayerReference::all_players(game){
                    if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player) {continue;}
                    if RoleSet::MafiaKilling.get_roles().contains(&player.role(game)) {continue;}
                    
                    let visits = player.untagged_night_visits_cloned(game).clone().into_iter()
                        .map(|mut v|
                            if self.hit_order_players.contains(&v.target) {
                                v.attack = true;
                                v
                            }else{
                                v
                            }
                        )
                        .collect();
                    player.set_night_visits(game, visits);
                }
            }
            //kill hit order players & remove hit
            Priority::Kill => {
                let mut players_to_remove_hit_order = VecSet::new();

                for player in PlayerReference::all_players(game){
                    if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player) {continue;}
                    if RoleSet::MafiaKilling.get_roles().contains(&player.role(game)) {continue;}
        
                    for visit in player.untagged_night_visits_cloned(game).clone(){
                        if !self.hit_order_players.contains(&visit.target) {continue;}
        
                        players_to_remove_hit_order.insert(visit.target);
                        
                        visit.target.try_night_kill_single_attacker(
                            player,
                            game,
                            crate::game::grave::GraveKiller::RoleSet(RoleSet::Mafia),
                            crate::game::attack_power::AttackPower::Basic,
                            false
                        );
                    }
                }
                for player in players_to_remove_hit_order.clone(){
                    MafiaHitOrders::remove_hit_order_player(game, player);
                }
            },
            _ => {}
        }
    }
    fn before_initial_role_creation(mut self, game: &mut Game) {
        //random weather or not to enable this modifier
        //if no mafia killing roles are enabled, then this modifier is active

        // If there is only 1 mafia insider, then deactivate this modifier instantly
        // Imagine getting the role godfather and then instantly switching to mafioso with no benefits 
        // because of this modifier
        let active = if InsiderGroupID::Mafia.players(game).len() == 1 {
            false
        }else{
            let possibilities = 1 + game.settings.enabled_roles.iter()
                .filter(|role|RoleSet::MafiaKilling.get_roles().contains(role))
                .count();
            rand::random::<usize>() % possibilities == 0
        };

        self.active = active;
        Modifiers::set_modifier(game, self.into());
        if !active {return;}


        // change all mafia killing roles to a random mafia support
        for player in PlayerReference::all_players(game){
            if
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, player) &&
                RoleSet::MafiaKilling.get_roles().contains(&player.role(game))
                
            {
                if let Some(role) = (RoleOutline::RoleOutlineOptions{options: vec1![RoleOutlineOption::RoleSet { role_set: RoleSet::MafiaSupport }]})
                    .get_random_role(
                        &game.settings.enabled_roles,
                        PlayerReference::all_players(game).map(|p|p.role(game)).collect::<Vec<_>>().as_slice()
                    )
                {
                    player.set_role_state(game, role.default_state());
                }
            }
        }
        
    }
}