//! Lyna the Light Charmer
use super::prelude::*;

pub const ID: u32 = 73318863;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), n, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e)
    //     and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
                // c:SetCardTarget(tc)
                card::set_card_target(d, c, tc);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_SET_CONTROL)
                effect::set_code(d, e1, EFFECT_SET_CONTROL);
                // e1:SetValue(tp)
                effect::set_value(d, e1, a.tp as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetCondition(s.ctcon)
                effect::set_condition(d, e1, Some(lua_fn(ctcon)));
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
/// `s.ctcon(e)`
fn ctcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local c=e:GetOwner()
    let c = effect::get_owner(d, e).nil();
    // local h=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    // return c:IsHasCardTarget(h)
    card::is_has_card_target(d, c, h)
}
