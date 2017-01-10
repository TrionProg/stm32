use basicTimer::BasicTimerRBTrait;
use basicTimer::BasicTimerConfigTrait;

pub trait BasicTimerTrait where
    Self: ::core::marker::Sized
{
    type BasicTimerRB:BasicTimerRBTrait;
    type Config:BasicTimerConfigTrait<BasicTimer=Self>;

    #[inline(always)]
    fn enable(&self, repeat:bool, period:u32, ){
        BasicTimerRB::set_opm(1);
        BasicTimerRB::set_arr(delay as u32);
        BasicTimerRB::set_cnt(0);
        BasicTimerRB::set_uie(1);
        Self::BasicTimerRB::set_cen(1);
    }

    #[inline(always)]
    fn disable(&self){
        Self::BasicTimerRB::set_cen(0);
    }

    fn callEvent(&self){
        Self::BasicTimerRB::set_ug(1);
    }

    fn isEvent(&self) -> bool{
        Self::BasicTimerRB::get_uif() != 0
    }

    fn config(&self, config:Self::Config){
        config.configure(self);
    }

    fn delay(&self, delay:u16, callback:Option<fn() -> ()>);
    fn repeat(&self, delay:u16, callback:Option<fn() -> ()>);
}
