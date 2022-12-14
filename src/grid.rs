pub struct Grid<T> {
    pub(crate) grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Create a new Grid<T> instance
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }
}
