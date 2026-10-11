//! Abyss-scale of the Kraken
use super::prelude::*;

pub const ID: u32 = 8719957;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MERMAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_MERMAIL))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_MERMAIL as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(400)
    effect::set_value(d, e2, 400);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // negate
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.negcon)
    effect::set_condition(d, e4, Some(condition(negcon)));
    // e4:SetOperation(s.negop)
    effect::set_operation(d, e4, Some(operation(negop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE
    //   and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    if a.rp == a.tp {
        return false;
    }
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    loc == LOCATION_MZONE as i64
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::send_to_grave(d, h, REASON_EFFECT, None, None);
    }
}
