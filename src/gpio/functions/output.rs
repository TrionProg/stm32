use ::gpio::pin::Pin;

pub trait OutputPin:Pin{
    #[inline(always)]
    fn enable_output();

    #[inline(always)]
    fn disable_output();

    #[inline(always)]
    fn turn_on();

    #[inline(always)]
    fn turn_off();
}

#[macro_export]
macro_rules! implement_output {
    ($pin:ident,enable_output : $enable_output:block,disable_output : $disable_output:block,turn_on : $turn_on:block,turn_off : $turn_off:block) => (
        impl OutputPin for $pin {
            fn enable_output() {
                Self::enable_pin();
                $enable_output
            }

            fn disable_output() {
                Self::disable_pin();
                $disable_output
            }

            fn turn_on(){
                $turn_on
            }

            fn turn_off(){
                $turn_off
            }
        }
    );
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
