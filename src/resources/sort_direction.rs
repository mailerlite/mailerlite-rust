//! Shared direction for resource lists with an explicit sort field.

/// Ascending or descending order for group and custom-field lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortDirection {
    /// Order from lowest to highest.
    Ascending,
    /// Order from highest to lowest.
    Descending,
}

impl SortDirection {
    pub(super) fn prefix(self) -> &'static str {
        match self {
            Self::Ascending => "",
            Self::Descending => "-",
        }
    }
}
