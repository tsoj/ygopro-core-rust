//! The Spell Absorbing Life
use super::prelude::*;

pub const ID: u32 = 99517131;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsType,TYPE_EFFECT)`
fn effect_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local rec=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,LOCATION_MZONE,nil)*400
    let rec = duel::get_matching_group_count(d, Some(&effect_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 400;
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec as u32);
    // if rec>0 then Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec) end
    if rec > 0 {
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_DEFENSE,POS_FACEUP_DEFENSE,true)
    duel::change_position(
        d,
        g,
        POS_FACEUP_ATTACK as u8,
        Some(POS_FACEUP_ATTACK as u8),
        Some(POS_FACEUP_DEFENSE as u8),
        Some(POS_FACEUP_DEFENSE as u8),
        true,
    );
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local rec=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,LOCATION_MZONE,nil)*400
    let rec = duel::get_matching_group_count(d, Some(&effect_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 400;
    // Duel.Recover(p,rec,REASON_EFFECT)
    duel::recover(d, p, rec, REASON_EFFECT, false, None);
}
