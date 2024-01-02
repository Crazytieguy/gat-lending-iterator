mod exact_size;
mod functions;
mod lending_iterator;
mod marker;
mod to_lending_iterator;
pub use self::exact_size::ExactSizeLendingIterator;
pub use self::functions::*;
pub use self::lending_iterator::LendingIterator;
pub use self::marker::{TrustedLen, TrustedLenIterator};
pub use self::to_lending_iterator::ToLendingIterator;
// MAYBE: IntoLendingIterator (chain, zip), FusedLendingIterator, DoubleEndedLendingIterator, Sum, Product, etc.
