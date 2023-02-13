use super::*;

create_role! { Sheriff
    let defense: 0;
    let roleblockable: true;
    let witchable: true;
    let suspicious: false;

    fn do_night_action(actor: &mut Player, game: &mut Game) {

        // if let Some(target) = actor.targets.get(0){
        //     target.suspicious
        // }
    }

    fn can_night_target(actor: &Player, target: &Player, game: &Game) -> bool {
        actor != target && actor.alive && target.alive
    }

    fn do_day_action(actor: &mut Player, target: &mut Player, game: &mut Game) {
        
    }

    fn can_day_target(actor: &Player, target: &Player, game: &Game) -> bool {
        false
    }

    fn convert_targets_to_visits(targets: &[PlayerIndex], game: &Game) -> Vec<Visit> {

        //TODO is there a way we can make this a "default" function. Because this function is going to be exaactly the same for many roles, so are other functions.
        //Even transporter it should stay the same, because we will just loop a certain number of times
        //we could just copy and paste
        let visits = Vec::new();

        if let Some(target) = targets.get(0) {
            visits.push(
                Visit{ target, astral: false, attack: false }
            );
        }
        
        visits
    }
    
    fn get_current_chat_groups(player: Player, game: &Game) -> Vec<ChatGroup> {
        todo!()
    }
}