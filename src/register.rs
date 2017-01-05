use volatile_register;

pub struct RW32{
     register:volatile_register::RW<u32>,
}

impl RW32{
    pub fn write(&mut self, value:u32){
        self.register.write(value);
    }

    pub fn read(&self) -> u32{
        self.register.read()
    }

    pub fn set_bits(&mut self, offset:u32, mask:u32, value:u32){
        let mut bits = self.register.read();
        bits&=!(mask<<offset);
        bits|=(value & mask)<<offset;
        self.register.write(bits);
    }

    pub fn get_bits(&self, offset:u32, mask:u32) -> u32{
        let mut bits = self.register.read();
        bits >> offset & mask
    }
}

pub struct RO32{
     register:volatile_register::RW<u32>,
}

impl RO32{
    pub fn read(&self) -> u32{
        self.register.read()
    }

    pub fn get_bits(&self, offset:u32, mask:u32) -> u32{
        let mut bits = self.register.read();
        bits >> offset & mask
    }
}

pub struct WO32{
     register:volatile_register::RW<u32>,
}

impl WO32{
    pub fn write(&mut self, value:u32){
        self.register.write(value);
    }

    pub fn set_bits(&mut self, offset:u32, mask:u32, value:u32){
        self.register.write((value & mask)<<offset);
    }
}
