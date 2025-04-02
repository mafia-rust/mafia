use std::collections::HashSet;

use crate::game::{chat::ChatMessageVariant, grave::Grave, player::PlayerReference, tag::Tag, Game};

#[derive(Default, Clone)]
pub struct LoveLinked {
    love_linked_players: HashSet<(PlayerReference, PlayerReference)>,
}
impl Game {
    pub fn love_linked(&self)->&LoveLinked{
        &self.love_linked
    }
    pub fn set_love_linked(&mut self, love_linked: LoveLinked){
        self.love_linked = love_linked;
    }
}
impl LoveLinked {
    pub fn add_love_link(game: &mut Game, player1: PlayerReference, player2: PlayerReference) {

        let mut love_linked = game.love_linked().clone();

        if 
            !love_linked.is_love_linked(player1, player2) &&
            !player1.night_died(game) &&
            !player2.night_died(game)
        {
            love_linked.love_linked_players.insert((player1, player2));
            game.set_love_linked(love_linked);
        
            player1.push_player_tag(game, player2, Tag::LoveLinked);
            player2.push_player_tag(game, player1, Tag::LoveLinked);

            if game.current_phase().is_night() {
                player1.push_night_message(game, ChatMessageVariant::YouAreLoveLinked { player: player2.index() });
                player2.push_night_message(game, ChatMessageVariant::YouAreLoveLinked { player: player1.index() });
            }
        }
    }

    pub fn get_links(game: &Game, player: PlayerReference) -> HashSet<PlayerReference> {
        game.love_linked().love_linked_players
            .iter()
            .filter(|(p1, p2)| *p1 == player || *p2 == player)
            .map(|(p1, p2)| if *p1 == player { *p2 } else { *p1 })
            .collect()
    }

    pub fn is_love_linked(&self, player1: PlayerReference, player2: PlayerReference) -> bool {
        self.love_linked_players.contains(&(player1, player2)) || self.love_linked_players.contains(&(player2, player1))
    }

    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        //die of a broken heart
        let links = LoveLinked::get_links(game, player);

        links.iter().for_each(|p| {
            if p.alive(game) {
                game.add_message_to_chat_group(
                    crate::game::chat::ChatGroup::All,
                    ChatMessageVariant::PlayerDiedOfABrokenHeart{player: p.index(), lover: player.index()}
                );
                p.die_and_add_grave(game, Grave::from_broken_heart(game, *p));
            }
        });
    }
}