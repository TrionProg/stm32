
#[macro_use]
pub mod pin;

pub use self::pin::Pin;

#[macro_use]
pub mod functions;

#[macro_export]
macro_rules! create_gpio{
    ($gpio_name:ident, gpio_rb:$gpio_rb:ident, enable_gpio:$enable_gpio:block, disable_gpio:$disable_gpio:block) => (
        use memory_map::addresses::{deref, deref_mut};
        use memory_map::addresses::$gpio_name as GPIO_RB_ADDRESS;
        use memory_map::gpio::gpio_rb::$gpio_rb as GpioRB;

        use ::stm32::gpio::Pin;

        static mut GPIO_COUNTER:u8=0;

        fn enable_gpio(){
            unsafe{
                if GPIO_COUNTER==0 {
                    $enable_gpio
                }

                GPIO_COUNTER+=1;
            }
        }

        fn disable_gpio(){
            unsafe{
                GPIO_COUNTER-=1;

                if GPIO_COUNTER==0 {
                    $disable_gpio
                }
            }
        }
    )
}
