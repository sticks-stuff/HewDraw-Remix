// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


#[no_mangle]
pub unsafe extern "Rust" fn pits_common(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    power_of_flight_cancel(boma, status_kind);
    upperdash_arm_whiff_freefall(fighter);
}


// Pits Power of Flight cancel
unsafe fn power_of_flight_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
        }
    }
}
 
unsafe fn upperdash_arm_jump_and_aerial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32, id: usize) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && frame > 6.0) {
        if (boma.is_situation(*SITUATION_KIND_GROUND) || WorkModule::is_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF))
        && frame > 28.0 {
            boma.check_jump_cancel(true) || boma.check_attack_hi4_cancel(false);
        }
    }
}

unsafe fn upperdash_arm_whiff_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && MotionModule::frame(fighter.module_accessor) >= MotionModule::end_frame(fighter.module_accessor) - 1.0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    upperdash_arm_jump_and_aerial_cancel(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame, id);
    pits_common(fighter, boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_PIT )]
pub fn pit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pit_frame(fighter)
    }
}

pub unsafe fn pit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}