//! Defender, the Magical Knight
use super::prelude::*;

pub const ID: u32 = 2525268;
pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,1)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 1);
    // summon success
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(addct)));
    effect::set_operation(d, e1, Some(operation(addc)));
    card::register_effect(d, c, e1, false);
    // destroy replace
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_value_fn(d, e2, lua_fn(value));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.addct(e,...,chk)`
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,COUNTER_SPELL)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, COUNTER_SPELL as i32);
    true
}
/// `s.addc(e,...)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsReason(REASON_REPLACE) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsRace(RACE_SPELLCASTER)
    !card::is_reason(d, c, REASON_REPLACE) && card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local count=eg:FilterCount(s.dfilter,nil)
        let eg = a.eg.clone().nil();
        let count = group::filter_count(d, &eg, &dfilter, NO_EX);
        // e:SetLabel(count)
        effect::set_label(d, a.e, &[count as i64]);
        // return count>0 and Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,count,REASON_COST)
        return count > 0 && duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, count as u32, REASON_COST);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.value(e,c)`
fn value(d: &mut Duel, a: Args) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsRace(RACE_SPELLCASTER)
    let c = a.c(1);
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_race(d, c, RACE_SPELLCASTER)
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local count=e:GetLabel()
    let count = effect::get_label(d, a.e);
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,count,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, count as u32, REASON_COST);
}
