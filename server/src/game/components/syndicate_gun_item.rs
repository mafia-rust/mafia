use crate::game::{attack_power::AttackPower, grave::GraveKiller, player::PlayerReference, role::Priority, role_list::RoleSet, visit::{Visit, VisitTag}, Game};

use super::night_visits::NightVisits;

#[derive(Default)]
pub struct SyndicateGunItem {
    player_with_gun: Option<PlayerReference>,
    gun_target: Option<PlayerReference>,
}

impl SyndicateGunItem {
    pub fn give_gun(game: &mut Game, player: PlayerReference) {
        game.syndicate_gun_item.player_with_gun = Some(player);
        game.syndicate_gun_item.gun_target = None;
    }

    pub fn target_gun(game: &mut Game, player: PlayerReference) {
        game.syndicate_gun_item.gun_target = Some(player);
    }

    //event listeners
    pub fn on_night_priority(game: &mut Game, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 
                let Some(gun_target) = game.syndicate_gun_item.gun_target else {return}; 
                NightVisits::add_visit(game, Visit::new(player_with_gun, gun_target, true));
            }
            Priority::Kill => {
                let targets: Vec<(PlayerReference, PlayerReference)> = NightVisits::all_visits(game)
                    .iter()
                    .filter(|visit| visit.tag == VisitTag::SyndicateGunItem)
                    .map(|visit| (visit.visitor, visit.target))
                    .collect();

                for (attacker, target) in targets {
                    target.try_night_kill_single_attacker(
                        attacker,
                        game,
                        GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::Basic,
                        false
                    );
                }
            }
            _ => {}
        }
    }
}