//! Dicelops
use super::prelude::*;

pub const ID: u32 = 62893810;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // dice
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DICE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,LOCATION_HAND,LOCATION_HAND)
    let g1 = duel::get_field_group(d, tp, LOCATION_HAND, LOCATION_HAND);
    // if chk==0 then return #g1~=0 end
    if a.chk == Some(0) {
        return !g1.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(g1), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local d=Duel.TossDice(tp,1)
    let result = duel::toss_dice(d, tp, 1, 0)[0];
    if result == 1 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, g.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    } else if result == 6 {
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
