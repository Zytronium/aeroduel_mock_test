use uuid::Uuid;

use crate::plane::Plane;
use crate::types::{PlaneId, UserId};

pub(crate) struct User {
    pub(crate) id: UserId,
    pub(crate) username: String,
    pub(crate) planes: Vec<PlaneId>, // array of plane ids
}

impl User {
    pub(crate) fn new(username: String) -> User {
        User {
            id: Uuid::new_v4(),
            username,
            planes: Vec::new(),
        }
    }

    pub(crate) fn link(&mut self, plane: &mut Plane) {
        if plane.pilot_id.is_some() {
            panic!("Plane already linked to another user.");
        }
        self.planes.push(plane.id);
        plane.set_pilot(self.id);
        println!("Plane {} linked to user {}.", plane.name, self.username);
    }
}
