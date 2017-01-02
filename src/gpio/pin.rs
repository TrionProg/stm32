pub trait Pin{
    type GpioRB:'static;

    #[inline(always)]
    fn enable_pin();

    #[inline(always)]
    fn disable_pin();

    #[inline(always)]
    fn get_index() -> u8;

    #[inline(always)]
    fn get_gpio_rb() -> &'static Self::GpioRB;

    #[inline(always)]
    fn get_gpio_rb_mut() -> &'static mut Self::GpioRB;
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

            fn get_index() -> u8{
                $index
            }

            fn get_gpio_rb() -> &'static Self::GpioRB{
                unsafe { deref(GPIO_RB_ADDRESS) }
            }

            fn get_gpio_rb_mut() -> &'static mut Self::GpioRB{
                unsafe { deref_mut(GPIO_RB_ADDRESS) }
            }
        }
    );
}
