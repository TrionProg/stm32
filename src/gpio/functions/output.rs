use ::gpio::pin::Pin;
use ::gpio::GpioRBTrait;

pub trait OutputPin:Pin{
    #[inline(always)]
    fn enable_output(){
        Self::enable_pin();
        Self::GpioRB::set_moder(Self::get_index(),0b01);
    }

    #[inline(always)]
    fn disable_output(){
        Self::disable_pin();
        Self::GpioRB::set_moder(Self::get_index(),0b00);
    }

    #[inline(always)]
    fn turn_on(){
        Self::GpioRB::set_bs(Self::get_index(),0b1);
    }

    #[inline(always)]
    fn turn_off(){
        Self::GpioRB::set_br(Self::get_index(),0b1);
    }
}

pub trait OutputFun{
    type Pin:OutputPin;

    #[inline(always)]
    fn enable(&self){
        Self::Pin::enable_output();
    }

    #[inline(always)]
    fn disable(&self){
        Self::Pin::disable_output();
    }

    fn enable_temporary(&self) -> TemporaryOutput<Self> where Self: Sized{
        TemporaryOutput::new(&self)
    }

    fn turn(&self, on:bool){
        if on {
            Self::Pin::turn_on();
        }else{
            Self::Pin::turn_off();
        }
    }
}


#[macro_export]
macro_rules! create_function_output {
    ($function_name:ident) => (

        pub struct $function_name<Pin:OutputPin>{
            output_pin:PhantomData<Pin>,
        }

        impl<Pin:OutputPin> $function_name<Pin>{
            pub const fn new() -> Self{
                $function_name::<Pin>{
                    output_pin:PhantomData,
                }
            }
        }

        impl<Pin:OutputPin> OutputFun for $function_name<Pin>{
            type Pin = Pin;
        }
    )
}

pub struct TemporaryOutput<'a, Output:'a + OutputFun>{
    output:&'a Output,
}

impl<'a, Output:OutputFun> TemporaryOutput<'a, Output>{
    pub fn new(output:&'a Output) -> Self{
        output.enable();

        TemporaryOutput{
            output:output,
        }
    }

    pub fn turn(&self,state:bool){
        self.output.turn(state);
    }
}

impl<'a, Output:OutputFun> Drop for TemporaryOutput<'a, Output>{
    fn drop(&mut self){
        self.output.disable();
    }
}
