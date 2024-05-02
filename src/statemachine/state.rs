use crate::interface::options::Options;
use crate::resource::location::Location;

use super::transition::Transition;

#[derive(Debug)]
pub struct State<S, A> {
    pub location: Location,
    pub scene: S,
    pub transitions: Options<Transition<A>>,
}
