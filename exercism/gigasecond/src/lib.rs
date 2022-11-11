
use time::{Duration, PrimitiveDateTime as DateTime};
const G: Duration = Duration::seconds(1_000_000_000);
pub fn after(start: DateTime) -> DateTime {
    start + G
}