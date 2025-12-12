use std::time::Duration;
use uuid::Uuid;
use crate::types::{MatchId, PlaneId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MatchState {
    Waiting,
    Active,
    Ended,
}

pub(crate) struct Match {
    pub(crate) id: MatchId,
    pub(crate) planes: Vec<PlaneId>, // store plane IDs here
    pub(crate) start_time: u64,
    pub(crate) duration: Duration,
    pub(crate) state: MatchState,
}

impl Match {
    pub(crate) fn new() -> Match {
        Match {
            id: Uuid::new_v4(),
            planes: Vec::new(),
            start_time: 0,
            duration: Duration::from_secs(0),
            state: MatchState::Waiting,
        }
    }

    pub(crate) fn add_plane(&mut self, plane_id: PlaneId) {
        if !self.planes.contains(&plane_id) {
            self.planes.push(plane_id);
        }
    }
}
