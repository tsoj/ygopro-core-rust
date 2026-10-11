//! Amplifier
use super::prelude::*;

pub const ID: u32 = 303660;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,CARD_JINZO),nil,nil,nil,nil,nil,EFFECT_FLAG_CANNOT_INACTIVATE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[CARD_JINZO]))),
        None,
        None,
        None,
        None,
        None,
        Some(EFFECT_FLAG_CANNOT_INACTIVATE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE),
    );
    // Immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e1:SetTarget(s.etarget)
    effect::set_target(d, e1, Some(effect_card_fn(etarget)));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy the equipped monster when this card leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Effect cannot be negated
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_DISABLE)
    effect::set_code(d, e3, EFFECT_CANNOT_DISABLE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.etarget(e,c)
fn etarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    let ec = card::get_equip_target(d, h);
    // return c:IsTrap() and ec and c:GetControler()==ec:GetControler()
    if !card::is_trap(d, c) {
        return false;
    }
    match ec {
        Some(ec) => card::get_controler(d, c) == card::get_controler(d, ec),
        None => false,
    }
}
// s.efilter(e,re)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler()==e:GetHandler():GetEquipTarget()
    let re = a.e(1);
    let rh = effect::get_handler(d, re).nil();
    let eh = effect::get_handler(d, a.e(0)).nil();
    Some(rh) == card::get_equip_target(d, eh)
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
