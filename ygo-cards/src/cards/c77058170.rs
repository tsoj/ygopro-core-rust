//! Underclock Taker
use super::prelude::*;

pub const ID: u32 = 77058170;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atkdown
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkfilter(c,g)`
fn atkfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:IsFaceup() and c:GetAttack()>0 and g:IsContains(c)
    card::is_faceup(d, c) && card::get_attack(d, c) > 0 && group::is_contains(g, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil,lg)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c, &lg)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELF as u64);
    // local g=Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil,lg)
    let g = duel::select_target(d, a.tp, Some(&|d, c| atkfilter(d, c, &lg)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_OPPO as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = match duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first() {
        Some(Value::Group(g)) => g.clone(),
        _ => crate::lua_error("attempt to index a nil value"),
    };
    // local tc2=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc2 = order.first().copied();
    // if tc1==tc2 then tc2=g:GetNext() end
    if tc1 == tc2 {
        tc2 = order.get(1).copied();
    }
    // if tc1:IsFaceup() and tc1:IsRelateToEffect(e) and tc2:IsFaceup() and tc2:IsRelateToEffect(e) then
    let tc1 = tc1.nil();
    if card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc1, a.e) {
        let tc2 = tc2.nil();
        if card::is_faceup(d, tc2) && card::is_relate_to_effect(d, tc2, a.e) {
            // local atk=tc1:GetAttack()
            let atk = card::get_attack(d, tc1);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-atk)
            effect::set_value(d, e1, -(atk as i64));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc2:RegisterEffect(e1)
            card::register_effect(d, tc2, e1, false);
        }
    }
}
