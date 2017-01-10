use timer::{TimerRBTrait,TimerTrait};

pub trait UIFRemapRBTrait:TimerRBTrait{
    /// UIF status bit remapping
    fn set_uifremap(value:u32);
    fn get_uifremap() -> u32;
    fn get_uif_copy() -> u32;
}

pub trait UIFRemapTrait:TimerTrait{
    type UIFRemapRB:UIFRemapRBTrait;

    fn set_uifremap(&self,enabled:bool){
        Self::UIFRemapRB::set_uifremap( if enabled {1} else {0} );
    }

    fn get_uifremap(&self) -> bool{
        Self::UIFRemapRB::get_uifremap() == 1
    }

    fn get_uif_copy(&self) -> bool{
        Self::UIFRemapRB::get_uif_copy() == 1
    }
}
