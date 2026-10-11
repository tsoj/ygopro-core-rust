//! Sergeant Electro
//! エレクトロ軍曹
use super::prelude::*;

pub const ID: u32 = 43359262;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,rc)`
fn filter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()~=5 and not rc:IsHasCardTarget(c)
    card::is_facedown(d, c) && card::get_sequence(d, c) != 5 && !card::is_has_card_target(d, rc, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=e:GetHandler() (the Lua passes e:GetHandler() as the filter argument)
    let rc = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(1-tp) and s.filter(chkc,e:GetHandler()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, rc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_SZONE,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, rc)), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWN as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_SZONE,1,1,nil,e:GetHandler())
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, rc)), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                // c:SetCardTarget(tc)
                card::set_card_target(d, c, tc);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CANNOT_TRIGGER)
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetCondition(s.rcon)
                effect::set_condition(d, e1, Some(condition(rcon)));
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
/// `s.rcon(e)`: `return e:GetOwner():IsHasCardTarget(e:GetHandler())`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
