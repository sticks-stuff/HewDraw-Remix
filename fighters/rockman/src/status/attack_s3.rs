use super::*;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rockman_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS3()
}

pub fn install() {
    install_status_scripts!(
        rockman_attack_s3_pre
    );
}