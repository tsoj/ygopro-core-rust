//! Worm Barses
use super::prelude::*;

pub const ID: u32 = 15658249;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsDefensePos() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_defense_pos(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsDefensePos,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_defense_pos(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsDefensePos() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_defense_pos(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangePosition(tc,0,0,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
            duel::change_position(d, tc, 0, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false);
        }
    }
}
