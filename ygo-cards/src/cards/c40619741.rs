//! Vylon Soldier
use super::prelude::*;

pub const ID: u32 = 40619741;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pos
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCanChangePosition()
    card::is_can_change_position(d, c)
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return e:GetHandler():GetEquipCount()>0 and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_equip_count(d, h) > 0 && duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,e:GetHandler():GetEquipCount(),nil)
    let h = effect::get_handler(d, a.e).nil();
    let max = card::get_equip_count(d, h) as u16;
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, max, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.ChangePosition(sg,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
    duel::change_position(d, sg, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false);
}
