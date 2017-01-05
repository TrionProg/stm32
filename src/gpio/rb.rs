pub trait GpioRBTrait{
    //Self
    fn get_gpio_rb() -> &'static Self;
    fn get_gpio_rb_mut() -> &'static mut Self;

    //Moder
    fn set_moder(index:u32,value:u32);
    fn get_moder(index:u32) -> u32;

    //Otyper
    fn set_ot(index:u32,value:u32);
    fn get_ot(index:u32) -> u32;

    //Ospeedr
    fn set_ospeedr(index:u32,value:u32);
    fn get_ospeedr(index:u32) -> u32;

    //Pupdr
    fn set_pupdr(index:u32,value:u32);
    fn get_pupdr(index:u32) -> u32;

    //Idr
    fn get_idr(index:u32) -> u32;

    //Odr
    fn set_odr(index:u32,value:u32);
    fn get_odr(index:u32) -> u32;

    //Bsrr
    fn set_bs(index:u32,value:u32);
    //fn set_br(index:u32,value:u32);

    //Lckr
    fn set_lck(index:u32,value:u32);
    fn get_lck(index:u32) -> u32;
    fn set_lckk(value:u32);
    fn get_lckk() -> u32;

    //Afrl/Afrh
    fn set_afr(index:u32,value:u32);
    fn get_afr(index:u32) -> u32;

    //Brr
    fn set_br(index:u32,value:u32);
}
