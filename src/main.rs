use std::collections::{HashMap, HashSet};
use std::time::Duration;
use uuid::Uuid;

fn main() {
    // Create simulated users
    let mut sim_user_1: User = User::new("Sim User 1".parse().unwrap());
    let mut sim_user_2: User = User::new("Sim User 2".parse().unwrap());
    
    // Create simulated planes
    let mut sim_plane_1: Plane = Plane::new("Foxtrot-4".parse().unwrap(), None);
    let mut sim_plane_2: Plane = Plane::new("Delta-7".parse().unwrap(), None);

    // Create match
    let mut sim_match: Match = Match::new();

    println!("Setup complete. Match, Users, and Planes created. Preparing to register and link planes...");

    // Register planes as online
    sim_plane_1.register();
    sim_plane_2.register();

    // Link planes to users
    sim_user_1.link(&mut sim_plane_1);
    sim_user_2.link(&mut sim_plane_2);

    println!("All planes online and linked. Preparing to start match...");
}

type MatchId = Uuid;
type PlaneId = Uuid;
type UserId = Uuid;

struct GameState {
    matches: HashMap<MatchId, Match>,
    ongoing_matches: HashSet<MatchId>,
    planes: HashMap<PlaneId, Plane>,
    users: HashMap<UserId, User>,
}

impl GameState {
    fn new() -> Self {
        Self {
            matches: HashMap::new(),
            ongoing_matches: HashSet::new(),
            planes: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn create_match(&mut self, m: Match) -> MatchId {
        let id = m.id;
        if m.state == "ONGOING" {
            self.ongoing_matches.insert(id);
        }
        self.matches.insert(id, m);
        id
    }

    fn set_match_state(&mut self, match_id: MatchId, new_state: &str) {
        let m = self.matches.get_mut(&match_id).expect("match not found");
        m.state = new_state.to_string();

        if m.state == "ONGOING" {
            self.ongoing_matches.insert(match_id);
        } else {
            self.ongoing_matches.remove(&match_id);
        }
    }

    fn list_ongoing_match_ids(&self) -> impl Iterator<Item = &MatchId> {
        self.ongoing_matches.iter()
    }

    fn get_match(&self, match_id: MatchId) -> Option<&Match> {
        self.matches.get(&match_id)
    }
}

struct Plane {
    id: PlaneId,
    name: String,
    pilot_id: Option<UserId>,
    joined_match_id: Option<MatchId>,
    online: bool,
    is_joined: bool,
    is_disqualified: bool,
    hits: u16,
    hits_taken: u16,
}

impl Plane {
    fn new(name: String, pilot_id: Option<Uuid>) -> Plane {
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

    fn register(&mut self) {
        self.online = true;
        println!("Plane {} registered as online.", self.name);
    }

    fn disconnect(&mut self) {
        self.online = false;
        self.is_joined = false;
        self.joined_match_id = None;
        // todo: consider disqualifying or resetting disqualified flag
        println!("Plane {} disconnected.", self.name);
    }

    fn join_match(&mut self, state: &mut GameState, match_id: MatchId) {
        let m = state.matches.get_mut(&match_id).expect("match not found");
        if m.state == "ACTIVE" {
            panic!("Cannot join an active match..")
        }
        if m.state == "ENDED" {
            panic!("Cannot join an ended match.")
        }
        if !m.planes.contains(&self.id) {
            m.planes.push(self.id);
        } else {
            println!("Plane {} tried to join a match twice!", self.name);
        }
        // todo: check if plane is already in another match

        self.is_joined = true;
        self.joined_match_id = Some(match_id);
        println!("Plane {} joined match {}", self.name, match_id);
    }

    fn leave_match(&mut self) {
        self.is_joined = false;
        self.joined_match_id = None;
        // todo: remove plane from match
        println!("Plane {} left match", self.name);
    }

    fn set_pilot(&mut self, pilot_id: Uuid) {
        if self.pilot_id.is_some() {
            panic!("Plane pilot id already set. To reset, use reset_pilot()");
        }
        if self.is_joined {
            panic!("Cannot set pilot while joined to match.");
        }
        self.pilot_id = Some(pilot_id);
    }

    fn reset_pilot(&mut self) {
        if self.is_joined {
            panic!("Cannot reset pilot while joined to match.");
        }
        self.pilot_id = None;
    }
}

struct User {
    id: Uuid,
    username: String,
    planes: Vec<Uuid>, // array of plane ids
}

impl User {
    pub(crate) fn link(&mut self, plane: &mut Plane) {
        if plane.pilot_id.is_some() {
            panic!("Plane already linked to another user.");
        }
        self.planes.push(plane.id);
        plane.set_pilot(self.id);
        println!("Plane {} linked to user {}.", plane.name, self.username);
    }
}

impl User {
    fn new(username: String) -> User {
        User {
            id: Uuid::new_v4(),
            username,
            planes: Vec::new(),
        }
    }
}

struct Match {
    id: Uuid,
    planes: Vec<Uuid>, // store plane IDs here
    start_time: u64,
    duration: Duration,
    state: String,
}

impl Match {
    fn new() -> Match {
        Match {
            id: Uuid::new_v4(),
            planes: Vec::new(),
            start_time: 0,
            duration: Duration::from_secs(0),
            state: String::from("WAITING"),
        }
    }
}
