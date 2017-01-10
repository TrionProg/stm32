
pub trait TimerRBTrait{
    /// Self
    fn get_rb() -> &'static Self;
    fn get_rb_mut() -> &'static mut Self;
}

pub trait TimerTrait{
    type TimerRB:TimerRBTrait;

    /// Timer enable
    fn enable_timer();
    fn disable_timer();
}

pub type Callback = Option<::alloc::boxed::Box<FnMut() -> ()>>;
pub type DMARequest = Option<()>;
