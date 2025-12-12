use uuid::Uuid;

use crate::game_state::GameState;
use crate::types::{MatchId, PlaneId, UserId};

pub(crate) struct Plane {
    pub(crate) id: PlaneId,
    pub(crate) name: String,
    pub(crate) pilot_id: Option<UserId>,
    pub(crate) joined_match_id: Option<MatchId>,
    pub(crate) online: bool,
    pub(crate) is_joined: bool,
    pub(crate) is_disqualified: bool,
    pub(crate) hits: u16,
    pub(crate) hits_taken: u16,
}

impl Plane {
    pub(crate) fn new(name: String, pilot_id: Option<UserId>) -> Plane {
        if name.is_empty() {
            panic!("Plane name cannot be empty");
        }

        Plane {
            id: Uuid::new_v4(),
            name,
            pilot_id,
            joined_match_id: None,
            online: false,
            is_joined: false,
            is_disqualified: false,
            hits: 0,
            hits_taken: 0,
        }
    }

    pub(crate) fn register(&mut self) {
        self.online = true;
        println!("Plane {} registered as online.", self.name);
    }

    pub(crate) fn disconnect(&mut self) {
        self.online = false;
        self.is_joined = false;
        // todo: consider removing plane from match
        // self.joined_match_id = None;
        // todo: consider disqualifying or resetting disqualified flag
        println!("Plane {} disconnected.", self.name);
    }

    pub(crate) fn join_match(&mut self, state: &mut GameState, match_id: MatchId) {
        // todo: check if plane is already in another match
        // temp easier check but less solid:
        if self.is_joined {
            panic!("Plane {} already joined to a match.", self.name);
        }

        state.add_plane_to_match(match_id, self.id);

        self.is_joined = true;
        self.joined_match_id = Some(match_id);
        println!("Plane {} joined match {}", self.name, match_id);
    }

    pub(crate) fn leave_match(&mut self) {
        self.is_joined = false;
        self.joined_match_id = None;
        // todo: remove plane from match
        println!("Plane {} left match", self.name);
    }

    pub(crate) fn set_pilot(&mut self, pilot_id: UserId) {
        if self.pilot_id.is_some() {
            panic!("Plane pilot id already set. To reset, use reset_pilot()");
        }
        if self.is_joined {
            panic!("Cannot set pilot while joined to match.");
        }
        self.pilot_id = Some(pilot_id);
    }

    pub(crate) fn reset_pilot(&mut self) {
        if self.is_joined {
            panic!("Cannot reset pilot while joined to match.");
        }
        self.pilot_id = None;
    }
}
