use super::*;
use smashline::*;

mod special_lw;

/// Prevents down b being reused
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        false.into()
    } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
        true.into()
    } else {
        false.into()
    }
}

#[smashline::fighter_init]
fn daisy_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_DAISY {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(daisy_init);
    special_lw::install();
}