mod exact_size;
mod functions;
mod lending_iterator;
mod to_lending_iterator;
pub use self::exact_size::{ExactSizeLendingIterator, HasNextLendingIterator};
pub use self::functions::*;
pub use self::lending_iterator::*;
pub use self::to_lending_iterator::ToLendingIterator;
// MAYBE: IntoLendingIterator (chain, zip), FusedLendingIterator, DoubleEndedLendingIterator, Sum, Product, etc.
