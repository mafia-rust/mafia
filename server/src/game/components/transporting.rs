
use crate::game::{
    Game,
    chat::ChatMessageVariant,
    event::on_midnight::MidnightVariables,
    player::PlayerReference,
    role::Role,
    visit::Visit,
};
use crate::vec_map::VecMap;


#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum TransportPriority {
    Transporting = 0,
    Warping = 1,
    Bodyguard = 2,
    None = 3
}


pub fn transport_priority(me: &PlayerReference, game: &mut Game) -> TransportPriority {
    match me.role(game) {
        Role::Transporter => TransportPriority::Transporting,
        Role::Warper | Role::Porter => TransportPriority::Warping,
        Role::Bodyguard => TransportPriority::Bodyguard,
        _ => TransportPriority::None
    }
}


pub fn transport(
    me: &PlayerReference, game: &mut Game, midnight_variables: &mut MidnightVariables,
    player_map: &VecMap<PlayerReference, PlayerReference>, send_message: bool, filter: &dyn Fn(&Visit) -> bool
) -> Vec<Visit> {
    if send_message {
        for p in player_map.keys() {
            p.push_night_message(midnight_variables, ChatMessageVariant::Transported);
        }
    }
    
    let self_transport_priority = transport_priority(me, game);
    let mut res = vec![];
    
    for player_ref in PlayerReference::all_players(game) {
        if transport_priority(&player_ref, game) <= self_transport_priority {continue;}

        let new_visits = player_ref.all_night_visits_cloned(game).clone().into_iter().map(|mut v| {
            let Some(new_target) = player_map.get(&v.target).filter(|_| filter(&v)) else {return v};
            v.target = *new_target;
            res.push(v);
            v
        }).collect();
        player_ref.set_night_visits(game, new_visits);
    }

    res
}