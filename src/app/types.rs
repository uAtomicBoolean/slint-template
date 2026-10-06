/// Returned by `update()` when a message is rejected.
#[derive(Debug)]
pub struct Rejected(#[allow(dead_code)] pub String);

/// The result of an `update()` call.
pub type Outcome = Result<(), Rejected>;
