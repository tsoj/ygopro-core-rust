//! Ghostrick Scare
use super::prelude::*;

pub const ID: u32 = 86516889;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWN as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,LOCATION_MZONE,0,1,99,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_MZONE, 0, 1, 99, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let count = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), count, 0, 0);
    true
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFacedown() and c:IsRelateToEffect(e)
    card::is_facedown(d, c) && card::is_relate_to_effect(d, c, e)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.filter,nil,e)
    let chain_g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &chain_g, &|d, c| filter(d, c, a.e), NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
    duel::change_position(d, g.clone(), POS_FACEUP_DEFENSE as u8, None, None, None, false);
    // local ct=g:FilterCount(Card.IsSetCard,nil,SET_GHOSTRICK)
    let ct = group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_GHOSTRICK as u16]), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local sg=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,ct,nil)
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, c| card::is_can_turn_set(d, c)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            ct as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.ChangePosition(sg,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, sg, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
