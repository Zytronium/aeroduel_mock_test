use std::collections::{HashMap, HashSet};

use crate::r#match::{Match, MatchState};
use crate::plane::Plane;
use crate::types::{MatchId, PlaneId, UserId};
use crate::user::User;

pub(crate) struct GameState {
    matches: HashMap<MatchId, Match>,
    ongoing_matches: HashSet<MatchId>,
    planes: HashMap<PlaneId, Plane>,
    users: HashMap<UserId, User>,
}

impl GameState {
    pub(crate) fn new() -> Self {
        Self {
            matches: HashMap::new(),
            ongoing_matches: HashSet::new(),
            planes: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub(crate) fn create_match(&mut self, m: Match) -> MatchId {
        let id = m.id;
        if m.state == MatchState::Active {
            self.ongoing_matches.insert(id);
        }
        self.matches.insert(id, m);
        id
    }

    pub(crate) fn set_match_state(&mut self, match_id: MatchId, new_state: MatchState) {
        let m = self.matches.get_mut(&match_id).expect("match not found");
        m.state = new_state;

        if m.state == MatchState::Active {
            self.ongoing_matches.insert(match_id);
        } else {
            self.ongoing_matches.remove(&match_id);
        }
    }

    pub(crate) fn list_ongoing_match_ids(&self) -> impl Iterator<Item = &MatchId> {
        self.ongoing_matches.iter()
    }

    pub(crate) fn get_match(&self, match_id: MatchId) -> Option<&Match> {
        self.matches.get(&match_id)
    }

    pub(crate) fn add_plane_to_match(&mut self, match_id: MatchId, plane_id: PlaneId) {
        let m = self.matches.get_mut(&match_id).expect("match not found");
        if m.state == MatchState::Active {
            panic!("Cannot join an active match..");
        }
        if m.state == MatchState::Ended {
            panic!("Cannot join an ended match.");
        }
        m.add_plane(plane_id);
    }
}
