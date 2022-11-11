// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const SECONDS_PER_YEAR: f64 = 31557600f64;
#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}
impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds: seconds as f64 }
    }
}
pub trait Planet {
    const PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_PER_YEAR / Self::PERIOD
    }
}
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
impl Planet for Mercury {
    const PERIOD: f64 = 0.2408467f64;
}
impl Planet for Venus {
    const PERIOD: f64 = 0.61519726f64;
}
impl Planet for Earth {
    const PERIOD: f64 = 1.0f64;
}
impl Planet for Mars {
    const PERIOD: f64 = 1.8808158f64;
}
impl Planet for Jupiter {
    const PERIOD: f64 = 11.862615f64;
}
impl Planet for Saturn {
    const PERIOD: f64 = 29.447498f64;
}
impl Planet for Uranus {
    const PERIOD: f64 = 84.016846f64;
}
impl Planet for Neptune {
    const PERIOD: f64 = 164.79132f64;
}
