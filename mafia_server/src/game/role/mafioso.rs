use super::*;

create_role! { Mafioso
    // TODO @ItsSammyM fill in this data
    let defense: 0;
    let roleblockable: true;
    let witchable: true;
    let suspicious: false;

    fn do_night_action(actor: &mut Player, game: &mut Game) {
        todo!()
    }

    fn can_night_target(actor: &Player, target: &Player, game: &Game) -> bool {
        todo!()
    }

    fn do_day_action(actor: &mut Player, target: &mut Player, game: &mut Game) {
        todo!()
    }

    fn can_day_target(actor: &Player, target: &Player, game: &Game) -> bool {
        todo!()
    }

    fn convert_targets_to_visits(targets: &[PlayerID], game: &Game) -> Vec<Visit> {
        todo!()
    }
    
    fn get_current_chat_groups(player: Player, game: &Game) -> Vec<Visit> {
        todo!()
    }
}