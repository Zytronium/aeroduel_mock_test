mod game_state;
mod r#match;
mod plane;
mod types;
mod user;

use crate::plane::Plane;
use crate::r#match::Match;
use crate::user::User;

fn main() {
    // Create simulated users
    let mut sim_user_1: User = User::new("Sim User 1".parse().unwrap());
    let mut sim_user_2: User = User::new("Sim User 2".parse().unwrap());

    // Create simulated planes
    let mut sim_plane_1: Plane = Plane::new("Foxtrot-4".parse().unwrap(), None);
    let mut sim_plane_2: Plane = Plane::new("Delta-7".parse().unwrap(), None);

    // Create match (not used yet in this demo)
    let _sim_match: Match = Match::new();

    println!("Setup complete. Match, Users, and Planes created. Preparing to register and link planes...");

    // Register planes as online
    sim_plane_1.register();
    sim_plane_2.register();

    // Link planes to users
    sim_user_1.link(&mut sim_plane_1);
    sim_user_2.link(&mut sim_plane_2);

    println!("All planes online and linked. Preparing to start match...");
}
