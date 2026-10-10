//! Ojama King
use super::prelude::*;

pub const ID: u32 = 90140980;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(12482652), fusion::Mat::Code(42941100), fusion::Mat::Code(79335209)]);
    // disable field
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    card::register_effect(d, c, e1, false);
}
fn disop(d: &mut Duel, a: Args) -> u32 {
    let tp = a.p(1);
    let c = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
    if c == 0 {
        return 0;
    }
    let mut dis1 = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(0), false);
    if c > 1 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        let dis2 = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(dis1), false);
        dis1 |= dis2;
        if c > 2 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            let dis3 = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(dis1), false);
            dis1 |= dis3;
        }
    }
    dis1
}
