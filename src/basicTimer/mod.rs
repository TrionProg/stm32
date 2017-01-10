#[macro_use]
mod rb;
pub use self::rb::BasicTimerRBTrait;

mod timer;
pub use self::timer::BasicTimerTrait;

mod config;
pub use self::config::BasicTimerConfigTrait;
pub use self::config::EventMode;
