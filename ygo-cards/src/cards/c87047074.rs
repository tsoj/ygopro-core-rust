//! Inzektor Crossbow - Zektarrow
use super::prelude::*;

pub const ID: u32 = 87047074;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_INZEKTOR))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_INZEKTOR as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // chain limit
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.chcon)
    effect::set_condition(d, e4, Some(condition(chcon)));
    // e4:SetOperation(s.chop)
    effect::set_operation(d, e4, Some(operation(chop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler()==e:GetHandler():GetEquipTarget()
    let rh = effect::get_handler(d, a.re.nil());
    let h = effect::get_handler(d, a.e).nil();
    rh == card::get_equip_target(d, h)
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimit(s.chlimit)
    duel::set_chain_limit(d, lua_fn(chlimit));
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // return ep==tp
    a.p(1) == a.p(2)
}
