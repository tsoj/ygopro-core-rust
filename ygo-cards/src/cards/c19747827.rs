//! Red-Eyes Black Dragon Sword
use super::prelude::*;

pub const ID: u32 = 19747827;
pub const INFO: ScriptInfo = ScriptInfo { material_race: Some(RACE_DRAGON), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // equip
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && chkc != h;
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let h = effect::get_handler(d, a.e).nil();
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, h.into());
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let Some(tc) = duel::first_target(d) else { return };
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_location(d, c, LOCATION_SZONE) || card::is_facedown(d, c) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    duel::equip(d, tp, c, tc, true, false);
    // e1: equip limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    effect::set_label_object(d, e1, Value::Card(tc));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    // e2: +1000 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 1000);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // e3: +500 ATK per Dragon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e3, false);
    // e4: the same for DEF
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    effect::get_label_object(d, a.e(0)).as_card() == a.card(1)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)), 0, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, NO_EX) as i64 * 500
}
