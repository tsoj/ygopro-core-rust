//! The Forceful Checkpoint
//! (検問)
use super::prelude::*;

pub const ID: u32 = 94861297;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, g.clone());
        // local tg=g:Filter(Card.IsMonster,nil)
        let tg = group::filter(d, &g, &|d, c| card::is_monster(d, c), NO_EX);
        // if #tg>0 and Duel.NegateAttack() then
        if !tg.is_empty() && duel::negate_attack(d) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
            // local hg=tg:Select(tp,1,1,nil)
            let hg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(hg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, hg, REASON_EFFECT | REASON_DISCARD, None, None);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
