//! Power of the Guardians
use super::prelude::*;

pub const ID: u32 = 1118137;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COUNTER);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(ctcon)));
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // atk/def
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
    // destroy replace
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_EQUIP);
    effect::set_code(d, e5, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e5, Some(cost_target(desreptg)));
    effect::set_operation(d, e5, Some(operation(desrepop)));
    card::register_effect(d, c, e5, false);
}
/// `s.ctcon(e,...)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // return Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc
    duel::get_attacker(d) == tc || duel::get_attack_target(d) == tc
}
/// `s.cttg(e,...,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,COUNTER_SPELL)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, COUNTER_SPELL as i32);
    true
}
/// `s.ctop(e,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandler():GetCounter(COUNTER_SPELL)*500
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, COUNTER_SPELL as u16) as i64 * 500
}
/// `s.desreptg(e,...,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=c:GetEquipTarget()
    let tg = card::get_equip_target(d, c);
    // if chk==0 then return tg and tg:IsReason(REASON_BATTLE|REASON_EFFECT) and Duel.IsCanRemoveCounter(...) end
    if a.chk == Some(0) {
        return match tg {
            Some(tg) => card::is_reason(d, tg, REASON_BATTLE | REASON_EFFECT) && duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 1, REASON_EFFECT),
            None => false,
        };
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.desrepop(e,...)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,1,REASON_EFFECT|REASON_REPLACE)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 1, REASON_EFFECT | REASON_REPLACE);
}
