pub trait BasicTimerRBTrait{
    //Self
    fn get_rb() -> &'static Self;
    fn get_rb_mut() -> &'static mut Self;

    //cr1
    /// Counter enable
    fn set_cen(value:u32);
    fn get_cen() -> u32;

    /// Update disable
    fn set_udis(value:u32);
    fn get_udis() -> u32;

    /// Update request source
    fn set_urs(value:u32);
    fn get_urs() -> u32;

    /// One-pulse mode
    fn set_opm(value:u32);
    fn get_opm() -> u32;

    /// Auto-reload preload enable
    fn set_arpe(value:u32);
    fn get_arpe() -> u32;

    //cr2
    /// Master mode selection
    fn set_mms(value:u32);
    fn get_mms() -> u32;

    //dier
    /// Update interrupt enable
    fn set_uie(value:u32);
    fn get_uie() -> u32;

    /// Update DMA request enable
    fn set_ude(value:u32);
    fn get_ude() -> u32;

    //sr
    /// Update interrupt flag
    fn get_uif() -> u32;

    //egr
    /// Update generation
    fn set_ug(value:u32);

    //cnt
    /// Counter value
    fn set_cnt(value:u32);
    fn get_cnt() -> u32;

    /// UIF Copy
    fn get_uif_copy() -> u32;

    //psc
    /// Prescaler value
    fn set_psc(value:u32);
    fn get_psc() -> u32;

    //arr
    /// Counter value to call event
    fn set_arr(value:u32);
    fn get_arr() -> u32;
}
