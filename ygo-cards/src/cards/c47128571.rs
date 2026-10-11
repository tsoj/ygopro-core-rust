//! Hidden Fangs of Revenge
use super::prelude::*;

pub const ID: u32 = 47128571;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_ONFIELD,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_ONFIELD, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,2,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, a.tp, LOCATION_ONFIELD as i32);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then Duel.SetChainLimit(aux.FALSE) end
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        duel::set_chain_limit(d, lua_fn(|_d, _a: Args| false));
    }
    true
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsLocation(LOCATION_GRAVE) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackBelow,c:GetDefense()),tp,0,LOCATION_MZONE,1,nil)
    if !(card::is_monster(d, c) && card::is_location(d, c, LOCATION_GRAVE)) {
        return false;
    }
    let def = card::get_defense(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, cc| card::is_faceup(d, cc) && card::is_attack_below(d, cc, def)),
        tp,
        0,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,Card.IsFacedown,tp,LOCATION_ONFIELD,0,2,2,nil)
    let dg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, 0, 2, 2, false, NO_EX).nil();
    // if #dg~=2 or Duel.Destroy(dg,REASON_EFFECT)~=2 or not Duel.NegateAttack() then return end
    if dg.len() != 2 || duel::destroy(d, dg, REASON_EFFECT, None, None) != 2 || !duel::negate_attack(d) {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if og:IsExists(s.filter,1,nil,tp) then
    if group::is_exists(d, &og, &|d, c| filter(d, c, tp), 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local tc=og:FilterSelect(tp,s.filter,1,1,nil,tp):GetFirst()
        let sel = group::filter_select(d, &og, tp, &|d, c| filter(d, c, tp), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,tc:GetDefense()),tp,0,LOCATION_MZONE,nil)
        let def = card::get_defense(d, tc);
        let g = duel::get_matching_group(
            d,
            Some(&|d, cc| card::is_faceup(d, cc) && card::is_attack_below(d, cc, def)),
            tp,
            0,
            LOCATION_MZONE,
            NO_EX,
        );
        // if Duel.Destroy(g,REASON_EFFECT)~=0 then
        if duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local turnp=Duel.GetTurnPlayer()
            let turnp = duel::get_turn_player(d);
            // Duel.SkipPhase(turnp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
            duel::skip_phase(d, turnp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
            // Duel.SkipPhase(turnp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
            duel::skip_phase(d, turnp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
        }
    }
}
