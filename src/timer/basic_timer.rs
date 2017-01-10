use timer::{TimerRBTrait, TimerTrait, Callback, DMARequest};
use core::marker::PhantomData;

pub trait BasicTimerRBTrait:TimerRBTrait{
    /// Counter enable
    fn set_cen(value:u32);
    fn get_cen() -> u32;

    /// One-pulse mode
    fn set_opm(value:u32);
    fn get_opm() -> u32;

    /// Auto-reload preload enable
    fn set_arpe(value:u32);
    fn get_arpe() -> u32;

    /// Prescaler value
    fn set_psc(value:u32);
    fn get_psc() -> u32;

    /// Update interrupt enable
    fn set_uie(value:u32);
    fn get_uie() -> u32;

    /// Update DMA request enable
    fn set_ude(value:u32);
    fn get_ude() -> u32;

    /// Update interrupt flag
    fn reset_uif();
    fn get_uif() -> u32;

    /// Update generation
    fn set_ug(value:u32);

    /// Counter value to call event
    fn set_arr(value:u32);
    fn get_arr() -> u32;
}

pub trait BasicTimerTrait:TimerTrait{
    type BasicTimerRB:BasicTimerRBTrait;

    fn get_callback() -> &'static Callback;
    fn get_callback_mut() -> &'static mut Callback;

    /// Enable Counter
    fn turn_counter(enabled:bool){
        Self::BasicTimerRB::set_cen( if enabled {1} else {0} );
    }

    fn is_counter_enabled() -> bool{
        Self::BasicTimerRB::get_cen() == 1
    }

    /// One-pulse mode
    fn turn_one_pulse_mode(enabled:bool){
        Self::BasicTimerRB::set_opm( if enabled {1} else {0} );
    }

    fn is_one_pulse_mode() -> bool{
        Self::BasicTimerRB::get_opm() == 1
    }

    /// Prescaler value
    fn set_prescaler(value:u16){
        Self::BasicTimerRB::set_psc(value as u32);
    }

    fn get_prescaler() -> u16{
        Self::BasicTimerRB::get_psc() as u16
    }

    /// Update interrupt flag
    fn turn_interrupt(enabled:bool){
        Self::BasicTimerRB::set_uie( if enabled {1} else {0} );
    }

    fn is_interrupt_enabled() -> bool{
        Self::BasicTimerRB::get_uie() == 1
    }

    /// Update DMA request enable
    fn turn_dma_request(enabled:bool){
        Self::BasicTimerRB::set_ude( if enabled {1} else {0} );
    }

    fn is_dma_request_enabled() -> bool{
        Self::BasicTimerRB::get_ude() == 1
    }

    /// Update interrupt flag
    fn reset_uif(){
        Self::BasicTimerRB::reset_uif();
    }

    fn is_uif() -> bool{
        Self::BasicTimerRB::get_uif() == 1
    }

    /// Update generation
    fn generate_update(){
        Self::BasicTimerRB::set_ug(1);
    }



    /* and arpe
    /// Counter value to call event
    fn set_arr(&self, value:u16){
        Self::BasicTimerRB::set_arr(value as u32);
    }

    fn get_arr(&self) -> u16{
        Self::BasicTimerRB::get_arr() as u16
    }
    */

    /// Config
    fn config(config:&BasicTimerConfig){
        Self::set_prescaler(config.prescaler-1);
    }

    fn delay(interval:u16, callback:Callback, dma_request:DMARequest){//Interrupt does not works =(
        let is_callback=callback.is_some();
        let is_dma_request=dma_request.is_some();

        *Self::get_callback_mut() = callback;
        //request?

        //Self::turn_one_pulse_mode(true);
        Self::turn_one_pulse_mode(false);
        Self::BasicTimerRB::set_arr(interval as u32);
        Self::generate_update();
        Self::reset_uif();

        //Self::turn_interrupt(is_callback);
        Self::turn_interrupt(true);
        Self::turn_dma_request(is_dma_request);

        Self::turn_counter(true);

        //Self::generate_update();
    }

    fn wait(interval:u16){
        Self::turn_one_pulse_mode(true);
        Self::BasicTimerRB::set_arr(interval as u32);
        Self::generate_update();
        Self::reset_uif();

        Self::turn_interrupt(false);
        Self::turn_dma_request(false);

        Self::turn_counter(true);

        while !Self::is_uif() {}

        Self::reset_uif();
    }
}

pub struct BasicTimer<Timer: BasicTimerTrait>{
    timer:PhantomData<Timer>,
    config:BasicTimerConfig,
}

impl<Timer: BasicTimerTrait> BasicTimer<Timer>{
    pub const fn new(config:BasicTimerConfig) -> Self{
        BasicTimer{
            timer:PhantomData,
            config:config,
        }
    }

    pub fn config(&self){
        Timer::enable_timer();
        Timer::config(&self.config);
    }

    pub fn delay(&self, interval:u16, callback:Callback, request:DMARequest){
        Timer::delay(interval, callback, request);
    }

    pub fn wait(&self, interval:u16){
        Timer::wait(interval);
    }

    pub fn stop(&self){
        Timer::turn_counter(false);
    }

    pub fn disable(&self){
        Timer::turn_counter(false);
        Timer::disable_timer();
    }
}

pub struct BasicTimerConfig{
    pub prescaler:u16,
    //arr_update_permanently???
}
