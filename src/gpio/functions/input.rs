use ::gpio::pin::Pin;

pub trait InputPin:Pin{
    #[inline(always)]
    fn config_input();
}

#[macro_export]
macro_rules! implement_input {
    ($pin:ident) => (
        impl InputPin for $pin {
            fn config_input() {
                Self::get_gpio_rb_mut().moder.modify(|_, w| { w.moder(Self::get_index(),0b00) });
            }
        }
    );
}

/*
    Pin::get_gpio_rb_address() _> usize

    gpio_rb_mut!() -> GPIO

    impl InputPin for E0<GPIO>{}
    E0<Gpio>
*/

/*
pub trait InputFun{
    type Pin:InputPin;

    fn config(pin:Pin){
        InputPin::config(pin); // and how to enable next?
    }
}
*/

/*
pub struct Input<Pin:InputPin>{
    pin:Pin,
}

impl Input{
    pub fn turn(&self, state:bool){
        //self.pin.get_register_block_mut.set....
        //точнее PinTrait::get_index(&self.pin)...
        //но можно и Pin::get_index()...
    }
}
*/
