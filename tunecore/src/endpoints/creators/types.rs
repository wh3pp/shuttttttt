/// Defines the sorting options for community song requests.
#[derive(Clone, Copy)]
pub enum SortBy {
    /// Sorts by popularity in descending order.
    Popularity,
    /// Sorts by the artists' revenue share rate in descending order.
    ShareRateDescending,
    /// Sorts by the artists' revenue share rate in ascending order.
    ShareRateAscending,
}
