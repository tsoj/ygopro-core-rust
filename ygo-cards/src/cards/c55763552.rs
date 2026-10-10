//! Dragon Piper
use super::prelude::*;

pub const ID: u32 = 55763552;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[50045299], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[50045299])
}
fn pfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_DEFENSE) && card::is_race(d, c, RACE_DRAGON)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        let pg = duel::get_matching_group(d, Some(&pfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        duel::change_position(d, pg, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
