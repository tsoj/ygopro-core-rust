//! Stairs of Mail
use super::prelude::*;

pub const ID: u32 = 19671102;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Change position
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
    // register names
    aux::global_check(d, ID, |d| {
        // s.name_list={}; s.name_list[0]={}; s.name_list[1]={}
        set_s_var(d, ID, 0, Value::Multi(Vec::new()));
        set_s_var(d, ID, 1, Value::Multi(Vec::new()));
        aux::add_values_reset(d, reset_name_list);
    });
}
/// `aux.AddValuesReset` function: `s.name_list[0]={}; s.name_list[1]={}`
fn reset_name_list(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Multi(Vec::new()));
    set_s_var(d, ID, 1, Value::Multi(Vec::new()));
}
/// `s.name_list[tp][code]`
fn name_listed(d: &Duel, tp: u8, code: u32) -> bool {
    match s_var(d, ID, tp as u64) {
        Value::Multi(v) => v.iter().any(|x| x.int() == code as i64),
        _ => false,
    }
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !card::is_set_card(d, c, &[SET_TINDANGLE as u16]) || !card::is_discardable(d, c, Some(REASON_EFFECT), None) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    !name_listed(d, tp, code)
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_can_change_position(d, c) && (card::is_position(d, c, POS_FACEUP_ATTACK) || card::is_position(d, c, POS_FACEDOWN_DEFENSE))
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, tp, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let dc = match group::get_first(d, &sel) {
        Some(dc) => dc,
        None => return,
    };
    if duel::send_to_grave(d, dc, REASON_DISCARD | REASON_EFFECT, None, None) == 0 {
        return;
    }
    // s.name_list[tp][dc:GetCode()]=true
    let code = card::get_code(d, dc)[0];
    let mut list = match s_var(d, ID, tp as u64) {
        Value::Multi(v) => v,
        _ => Vec::new(),
    };
    list.push(Value::Int(code as i64));
    set_s_var(d, ID, tp as u64, Value::Multi(list));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_matching_card(d, tp, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, Some(0), Some(0), Some(POS_FACEUP_ATTACK as u8), false);
    }
}
