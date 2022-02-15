use crate::docker::docker::Id;

#[derive(Debug)]
pub struct Container {
    pub id: Id,
    pub name: String,
    pub state: State,
}

#[derive(Debug)]
pub struct State {
    pub pid: u32,
}
