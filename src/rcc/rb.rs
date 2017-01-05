pub trait RccRBTrait{
    //Self
    fn get_rcc() -> &'static Self;
    fn get_rcc_mut() -> &'static mut Self;
}
