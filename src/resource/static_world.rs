use crate::resource::state::State;

use super::location::Location;

pub trait StaticWorld {
    fn get_state(&self, location: &Location) -> &State;
}
