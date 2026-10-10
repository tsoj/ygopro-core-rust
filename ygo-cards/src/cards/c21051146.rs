//! Blast Magician
use super::prelude::*;

pub const ID: u32 = 21051146;
pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

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
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
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
/// `math.ceil(c:GetAttack()/700)`, and 1 when that is 0
fn counter_cost(d: &mut Duel, c: CardId) -> i64 {
    let mut ct = (card::get_attack(d, c) as f64 / 700.0).ceil() as i64;
    if ct == 0 {
        ct = 1;
    }
    ct
}
/// `s.filter(c,cc,tp)`
fn filter(d: &mut Duel, c: CardId, cc: CardId, tp: u8) -> bool {
    // local ct=math.ceil(c:GetAttack()/700)
    // if ct==0 then ct=1 end
    let ct = counter_cost(d, c);
    // return c:IsFaceup() and cc:IsCanRemoveCounter(tp,COUNTER_SPELL,ct,REASON_COST)
    card::is_faceup(d, c) && card::is_can_remove_counter(d, cc, tp, COUNTER_SPELL as u16, ct as u16, REASON_COST)
}
/// `s.destg(e,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,e:GetHandler(),tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && {
            let h = effect::get_handler(d, e).nil();
            filter(d, chkc, h, tp)
        };
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, h, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e:GetHandler(),tp)
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, h, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local ct=math.ceil(g:GetFirst():GetAttack()/700)
    // if ct==0 then ct=1 end
    let first = group::get_first(d, &g).nil();
    let ct = counter_cost(d, first);
    // e:GetHandler():RemoveCounter(tp,COUNTER_SPELL,ct,REASON_COST)
    let h = effect::get_handler(d, e).nil();
    card::remove_counter(d, h, tp, COUNTER_SPELL as u16, ct as u16, REASON_COST);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
