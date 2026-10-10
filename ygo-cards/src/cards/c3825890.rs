//! Gravekeeper's Visionary
use super::prelude::*;

pub const ID: u32 = 3825890;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    let _e2 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // atk up
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    card::register_effect(d, c, e3, false);
    // Destroy replace
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    card::register_effect(d, c, e4, false);
}
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GRAVEKEEPERS) and (c:IsControler(tp) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GRAVEKEEPERS) and c:IsMonster()
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::is_monster(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 200
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
        true
    } else {
        false
    }
}
