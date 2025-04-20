use crate::game::{chat::ChatMessageVariant, event::on_midnight::{MidnightVariables, OnMidnight, OnMidnightPriority}, player::PlayerReference, Game};

pub struct NightProtected;
impl NightProtected{
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority){
        if priority != OnMidnightPriority::Investigative {return};

        for player in PlayerReference::all_players(game){
            for protected in player.protected_players(midnight_variables).clone() {
                if protected.night_attacked(midnight_variables){
                    player.push_night_message(midnight_variables, ChatMessageVariant::YouGuardedSomeone);
                    protected.push_night_message(midnight_variables, ChatMessageVariant::YouWereGuarded);
                }
            }
        }
    }
}