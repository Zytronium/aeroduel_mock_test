use std::time::Duration;

fn main() {
    // Create users
    let user1 = User::new("Player1".to_string());
    let user2 = User::new("Player2".to_string());

    // Create planes
    let mut plane1 = Plane::new("F16".to_string(), user1.id.clone());
    let mut plane2 = Plane::new("MiG29".to_string(), user2.id.clone());

    // Create match
    let mut game_match = Match::new();

    // Register planes as online
    plane1.register();
    plane2.register();

    // Join one plane to match
    plane1.join_match();

    // Add planes to match
    game_match.planes.push(plane1);
    game_match.planes.push(plane2);

    // Print match state
    println!("Match state: {}", game_match.state);
    println!("Number of planes: {}", game_match.planes.len());
    for plane in game_match.planes.iter() {
        println!("Plane: {}, Online: {}, Joined: {}, id: {}",
                 plane.name, plane.online, plane.is_joined, plane.id);
    }
}

struct Plane {
    id: String,
    name: String,
    pilot_id: String,
    online: bool,
    is_joined: bool,
    is_disqualified: bool,
    hits: u16,
    hits_taken: u16,
}

struct User {
    id: String,
    username: String,
    planes: Vec<String> // array of plane ids
}

impl User {
    fn new(username: String) -> User {
        User {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            planes: Vec::new(),
        }
    }
}

struct Match {
    id: String,
    planes: Vec<Plane>,
    start_time: u64,
    duration: Duration,
    state: String,
}

impl Match {
    fn new() -> Match {
        Match {
            id: uuid::Uuid::new_v4().to_string(),
            planes: Vec::new(),
            start_time: 0,
            duration: Duration::from_secs(0),
            state: String::from("WAITING"),
        }
    }
}

impl Plane {
    fn new(name: String, pilot_id: String,) -> Plane {
        if name.is_empty() { panic!("Plane name cannot be empty"); }
        if pilot_id.is_empty() { panic!("Plane pilot id cannot be empty"); }
        // todo: ensure pilot_id is valid

        Plane {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            pilot_id,
            online: false,
            is_joined: false,
            is_disqualified: false,
            hits: 0,
            hits_taken: 0,
        }
    }

    fn register(&mut self) {
        self.online = true;
        println!("Plane {} registered", self.name);
    }

    fn join_match(&mut self) {
        self.is_joined = true;
        // todo: join match
        println!("Plane {} joined match", self.name);
    }
}
