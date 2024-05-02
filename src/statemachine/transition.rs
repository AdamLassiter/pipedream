use crate::resource::location::Location;

#[derive(Debug, Clone)]
pub struct Transition<A> {
    pub next: Location,
    pub action: A,
}
