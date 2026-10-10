//! Mythical Beast Cerberus
use super::prelude::*;

pub const ID: u32 = 55424270;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // add counter
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // attackup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(attackup));
    card::register_effect(d, c, e2, false);
    // remove counter
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
/// `s.acop(e,...)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():AddCounter(COUNTER_SPELL,1)
            let h = effect::get_handler(d, a.e).nil();
            card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
        }
    }
}
/// `s.attackup(e,c)`
fn attackup(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(COUNTER_SPELL)*500
    card::get_counter(d, c, COUNTER_SPELL as u16) as i64 * 500
}
/// `s.condition(e,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.operation(e,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local n=c:GetCounter(COUNTER_SPELL)
    let n = card::get_counter(d, c, COUNTER_SPELL as u16);
    // if n~=0 then c:RemoveCounter(tp,COUNTER_SPELL,n,REASON_EFFECT) end
    if n != 0 {
        card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, n as u16, REASON_EFFECT);
    }
}
