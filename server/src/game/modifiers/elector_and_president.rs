use rand::{thread_rng, Rng};

use crate::game::{phase::PhaseType, player::PlayerReference, tag::Tag, Game};

use super::{ModifierState, ModifierTrait, Modifiers};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ElectorAndPresident {
    pub elector: PlayerReference,
    pub president: President,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub enum President {
    #[default]
    None,
    StillDeciding { candidate_to_be: Option<PlayerReference> },
    Candidate { candidate: PlayerReference },
    President { president: PlayerReference },
}

impl ElectorAndPresident {
    fn set_random_elector(&mut self, game: &mut Game) {
        self.set_elector(game, PlayerReference::new_unchecked(thread_rng().gen_range(0..game.players.len()) as u8));
        self.clear_president(game);
    }
    fn set_elector(&mut self, game: &mut Game, elector: PlayerReference) {
        self.elector = elector;

        for player in PlayerReference::all_players(game) {
            player.push_player_tag(game, self.elector, Tag::Elector);
        }
        Modifiers::set_modifier_state(game, ModifierState::ElectorAndPresident(self.clone()));
    }
    fn start_deciding_president(&mut self, game: &mut Game) {
        self.president = President::StillDeciding { candidate_to_be: None };
        Modifiers::set_modifier_state(game, ModifierState::ElectorAndPresident(self.clone()));
    }
    fn set_candidate_decision(&mut self, game: &mut Game, candidate_to_be: Option<PlayerReference>) {
        if let President::StillDeciding { candidate_to_be: Some(old_candidate) } = self.president {
            for player in PlayerReference::all_players(game) {
                player.remove_player_tag(game, old_candidate, Tag::PresidentialCandidate);
            }
        }
        if let Some(candidate) = candidate_to_be {
            for player in PlayerReference::all_players(game) {
                player.push_player_tag(game, candidate, Tag::PresidentialCandidate);
            }
        }
        self.president = President::StillDeciding { candidate_to_be };
        Modifiers::set_modifier_state(game, ModifierState::ElectorAndPresident(self.clone()));
    }
    fn finalize_candidate(&mut self, game: &mut Game) {
        let candidate = match self.president {
            President::StillDeciding { candidate_to_be: Some(candidate) } => candidate,
            _ => PlayerReference::new_unchecked(thread_rng().gen_range(0..game.players.len()) as u8),
        };
        for player in PlayerReference::all_players(game) {
            player.push_player_tag(game, candidate, Tag::PresidentialCandidate);
        }
        self.president = President::Candidate { candidate };
        Modifiers::set_modifier_state(game, ModifierState::ElectorAndPresident(self.clone()));
    }
    fn clear_president(&mut self, game: &mut Game) {
        self.president = President::None;
        Modifiers::set_modifier_state(game, ModifierState::ElectorAndPresident(self.clone()));
    }
}

impl ModifierTrait for ElectorAndPresident{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::ElectorAndPresident
    }
    fn on_game_start(mut self, game: &mut Game) {
        self.set_random_elector(game);
    }
    fn on_phase_start(mut self, game: &mut Game, phase: PhaseType) {
        match phase {
            PhaseType::Obituary => {
                // Remove old elector
                for player in PlayerReference::all_players(game) {
                    player.remove_player_tag(game, self.elector, Tag::Elector);
                }
                // Remove old president
                if let President::President { president: old_president } = self.president {
                    for player in PlayerReference::all_players(game) {
                        player.remove_player_tag(game, old_president, Tag::President);
                    }
                }
                self.set_elector(game, PlayerReference::new_unchecked((self.elector.index() + 1) % game.players.len() as u8));
            }
            PhaseType::Discussion => {
                self.start_deciding_president(game);
            }
            PhaseType::Nomination => {
                self.finalize_candidate(game);
            }
            _ => {}
        }
    }
}
