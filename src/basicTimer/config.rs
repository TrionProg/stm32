use basicTimer::BasicTimerTrait;

pub enum EventMode{
    Once,
    Repeat,
}

pub trait BasicTimerConfigTrait{
    type BasicTimer:BasicTimerTrait;

    fn configure(&self, timer:&Self::BasicTimer);
}
