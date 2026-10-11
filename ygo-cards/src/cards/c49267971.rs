//! Blind Spot Strike
use super::prelude::*;

pub const ID: u32 = 49267971;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsPosition,tp,0,LOCATION_MZONE,1,nil,POS_FACEUP_DEFENSE)
    //     and Duel.IsExistingTarget(Card.IsPosition,tp,LOCATION_MZONE,0,1,nil,POS_FACEUP_ATTACK) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_OPPO as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsPosition,tp,0,LOCATION_MZONE,1,1,nil,POS_FACEUP_DEFENSE)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELF as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsPosition,tp,LOCATION_MZONE,0,1,1,nil,POS_FACEUP_ATTACK)
    let _g2 = duel::select_target(d, a.tp, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let mut tc2 = group::get_first(d, &g);
    // if tc1==tc2 then tc2=g:GetNext() end
    if tc1 == tc2 {
        tc2 = group::iter(d, &g).get(1).copied();
    }
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and tc2:IsRelateToEffect(e) and tc2:IsFaceup() then
    let tc1 = tc1.nil();
    let tc2 = tc2.nil();
    if card::is_relate_to_effect(d, tc1, a.e) && card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc2, a.e) && card::is_faceup(d, tc2) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc1:GetDefense())
        let def = card::get_defense(d, tc1) as i64;
        effect::set_value(d, e1, def);
        // tc2:RegisterEffect(e1)
        card::register_effect(d, tc2, e1, false);
    }
}
