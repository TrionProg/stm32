use ::gpio::GpioRBTrait;

pub trait Pin{
    type GpioRB:GpioRBTrait + 'static;

    #[inline(always)]
    fn enable_pin();

    #[inline(always)]
    fn disable_pin();

    #[inline(always)]
    fn get_index() -> u32;
}

#[macro_export]
macro_rules! create_pin {
    ($pin:ident,$index:expr) => (
        pub struct $pin;

        impl Pin for $pin{
            type GpioRB = GpioRB;

            fn enable_pin(){
                enable_gpio();
            }

            fn disable_pin(){
                disable_gpio();
            }

            fn get_index() -> u32{
                $index
            }
        }
    );
}
