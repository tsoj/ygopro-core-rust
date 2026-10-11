//! Battle Break
//! バトル・ブレイク
use super::prelude::*;

pub const ID: u32 = 22047978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chk==0 then return tg:IsOnField() end
    if a.chk == Some(0) {
        let t = tg.nil();
        return card::is_on_field(d, t);
    }
    // Duel.SetTargetCard(tg)
    if let Some(t) = tg {
        duel::set_target_card(d, t);
    }
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 then
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg.map(CardSet::from_card), 1, 0, 0);
    }
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic() and c:IsMonster()
    !card::is_public(d, c) && card::is_monster(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local sel=1
        // local g=Duel.GetMatchingGroup(s.cfilter,1-tp,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), 1 - a.tp, LOCATION_HAND, 0, NO_EX);
        // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, aux::stringid(ID, 0));
        // if #g>0 then
        let sel = if !g.is_empty() {
            // sel=Duel.SelectOption(1-tp,1213,1214)
            duel::select_option(d, 1 - a.tp, true, &[1213, 1214])
        } else {
            // sel=Duel.SelectOption(1-tp,1214)+1
            duel::select_option(d, 1 - a.tp, true, &[1214]) + 1
        };
        // if sel==0 then
        if sel == 0 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONFIRM)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONFIRM as u64);
            // local cg=g:Select(1-tp,1,1,nil)
            let cg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
            // Duel.ConfirmCards(tp,cg)
            duel::confirm_cards(d, a.tp, cg);
            // Duel.ShuffleHand(1-tp)
            duel::shuffle_hand(d, 1 - a.tp);
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            // return
            return;
        }
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:CanAttack() and not tc:IsStatus(STATUS_ATTACK_CANCELED)
    //   and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::can_attack(d, tc)
        && !card::is_status(d, tc, STATUS_ATTACK_CANCELED)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
    {
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
