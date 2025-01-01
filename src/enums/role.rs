#[derive(Debug)]
pub enum Role {
    Forager,
    Guard,
    Builder
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}