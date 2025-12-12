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
    // test();
}

fn test() {
    // Create users
    let user1 = User::new("Player1".to_string());
    let user2 = User::new("Player2".to_string());

    // Create planes
    let mut plane1 = Plane::new("F16".to_string(), Some(user1.id));
    let mut plane2 = Plane::new("MiG29".to_string(), Some(user2.id));

    // Create match
    let mut game_match = Match::new();

    // Register planes as online
    plane1.register();
    plane2.register();

    // Join one plane to match
    plane1.join_match(game_match.id);

    // Add planes to match (store IDs)
    game_match.planes.push(plane1.id);
    game_match.planes.push(plane2.id);

    // Print match state
    println!("Match state: {}", game_match.state);
    println!("Number of planes: {}", game_match.planes.len());
    for plane_id in game_match.planes.iter() {
        println!("Plane id in match: {}", plane_id);
    }

    // If you want plane details, you need a lookup table (e.g. HashMap<Uuid, Plane>) or print from variables:
    println!(
        "Plane1: {}, Online: {}, Joined: {}, id: {}",
        plane1.name, plane1.online, plane1.is_joined, plane1.id
    );
    println!(
        "Plane2: {}, Online: {}, Joined: {}, id: {}",
        plane2.name, plane2.online, plane2.is_joined, plane2.id
    );
}

struct Plane {
    id: Uuid,
    name: String,
    pilot_id: Option<Uuid>,
    joined_match_id: Option<Uuid>,
    online: bool,
    is_joined: bool,
    is_disqualified: bool,
    hits: u16,
    hits_taken: u16,
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

    fn join_match(&mut self, match_id: Uuid) {
        self.is_joined = true;
        self.joined_match_id = Some(match_id);
        // todo: add plane to match
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
