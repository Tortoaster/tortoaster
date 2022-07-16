#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Action {
    Flip,
    Place(usize),
    Take(usize),
}
