use hypersig::Segment;
use hypersig_ui::{SortOrder, TableState};
use serde::{Deserialize, Serialize};

/// The columns a `ServerTable` sorts by.
#[derive(Segment, Serialize, Deserialize, Debug, PartialEq)]
enum By {
    Email,
    Amount,
}

// The first state of a page whose rows arrive sorted by amount, largest first.
let state = TableState::default().with_sorting(&By::Amount, SortOrder::Descending);
assert_eq!(state.sorting::<By>(), Some((By::Amount, SortOrder::Descending)));

// An unsorted state has no sort to read.
assert_eq!(TableState::default().sorting::<By>(), None);
