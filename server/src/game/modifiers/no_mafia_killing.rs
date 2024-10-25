use crate::{game::{components::revealed_group::RevealedGroupID, player::PlayerReference, role::Priority, role_list::RoleSet, tag::Tag, Game}, packet::ToClientPacket, vec_map::VecMap, vec_set::VecSet};

use super::{ModifierTrait, ModifierType, Modifiers};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct NoMafiaKilling{
    hit_order_players: VecSet<PlayerReference>,
    hit_order_vote: VecMap<PlayerReference, PlayerReference>,
}

impl From<&NoMafiaKilling> for ModifierType{
    fn from(_: &NoMafiaKilling) -> Self {
        ModifierType::NoMafiaKilling
    }
}

impl NoMafiaKilling{
    pub fn is_player_marked(game: &mut Game, player: PlayerReference)->bool{
        let Some(modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return false};
        modifier.hit_order_players.contains(&player)
    }

    pub fn mark_vote_action(game: &mut Game, voter: PlayerReference, target: Option<PlayerReference>){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return};
        
        if !voter.alive(game) {return}
        if !RevealedGroupID::Mafia.is_player_in_revealed_group(game, voter) {return}
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
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return};
        for player in modifier.hit_order_vote.keys(){
            player.send_packet(game, ToClientPacket::YourHitOrderVote{player: None});
        }
        modifier.hit_order_vote.clear();
        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn hit_order_voted_player(game: &mut Game)->Option<PlayerReference>{
        let Some(modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return None};
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
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return};
        if modifier.hit_order_players.insert(player).is_some() {return;}

        for insider in RevealedGroupID::Mafia.players(game).clone(){
            insider.push_player_tag(game, player, Tag::GodfatherBackup);
        }
        Modifiers::set_modifier(game, modifier.into());
    }
    pub fn remove_hit_order_player(game: &mut Game, player: PlayerReference){
        let Some(mut modifier) = Modifiers::get_modifier_inner::<Self>(game, ModifierType::NoMafiaKilling).cloned() else {return};
        modifier.hit_order_players.remove(&player);
        for insider in RevealedGroupID::Mafia.players(game).clone(){
            insider.remove_player_tag(game, player, Tag::GodfatherBackup);
        }
        Modifiers::set_modifier(game, modifier.into());
    }
}

impl ModifierTrait for NoMafiaKilling{
    fn before_phase_end(self, game: &mut Game, phase:crate::game::phase::PhaseType) {
        if game.day_number() == 1 {return;}
        if phase != crate::game::phase::PhaseType::Night {return}
        let Some(player) = Self::hit_order_voted_player(game) else {return};
        NoMafiaKilling::add_hit_order_player(game, player);
        NoMafiaKilling::remove_all_votes(game);
    }
    fn on_night_priority(self, game: &mut Game, priority: Priority){
        if game.day_number() == 1 {return;}
        if priority != Priority::Kill {return;}
        let mut players_to_remove_hit_order = VecSet::new();

        for player in PlayerReference::all_players(game){
            if !RevealedGroupID::Mafia.is_player_in_revealed_group(game, player) {continue;}

            for visit in player.night_visits(game).clone(){
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
            
            //set visits to attacking visits
            let visits = player.night_visits(game).clone().into_iter()
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
        for player in players_to_remove_hit_order.clone(){
            NoMafiaKilling::remove_hit_order_player(game, player);
        }
    }
}