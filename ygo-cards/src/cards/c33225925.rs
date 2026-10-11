//! Milla the Temporal Magician
use super::prelude::*;

pub const ID: u32 = 33225925;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // confirm
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetChainLimit(s.chainlimit)
    duel::set_chain_limit(d, lua_fn(chainlimit));
    true
}
/// `s.chainlimit(e,rp,tp)`
fn chainlimit(d: &mut Duel, a: Args) -> bool {
    // return tp==rp or not e:IsHasType(EFFECT_TYPE_ACTIVATE)
    let e = a.e(0);
    let rp = a.p(1);
    let tp = a.p(2);
    tp == rp || !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFacedown() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_facedown(d, tc) {
            // Duel.ConfirmCards(tp,tc)
            duel::confirm_cards(d, a.tp, tc);
        }
    }
}
