//! Dice It
use super::prelude::*;

pub const ID: u32 = 8868767;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_REMOVE+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_REMOVE + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=false
        // if turn_p==tp then
        //     res=Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,nil)
        // else
        //     res=Duel.IsPlayerCanDiscardDeck(tp,1)
        // end
        // return res
        return if turn_p == tp {
            duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c)), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX)
        } else {
            duel::is_player_can_discard_deck(d, tp, 1)
        };
    }
    // e:SetLabel(turn_p)
    effect::set_label(d, a.e, &[turn_p as i64]);
    // if turn_p==tp then
    if turn_p == tp {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 1);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 1);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local res=Duel.TossDice(tp,1)
    let res = duel::toss_dice(d, tp, 1, 0)[0];
    // if e:GetLabel()==tp then
    if effect::get_label(d, a.e) == tp as i64 {
        // Activated during your turn
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local rg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,res,res,nil)
        let rg = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c)), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, res as u16, res as u16, false, NO_EX).nil();
        // if #rg==0 then return end
        if rg.is_empty() {
            return;
        }
        // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)==res and res==1 then
        if duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) == res && res == 1 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardDeck(tp,6,REASON_EFFECT)
            duel::discard_deck(d, tp, 6, REASON_EFFECT);
        }
    } else {
        // Activated during your opponent's turn
        // if Duel.DiscardDeck(tp,res,REASON_EFFECT)==res and res==6 then
        if duel::discard_deck(d, tp, res as u16, REASON_EFFECT) == res && res == 6 {
            // local og=Duel.GetOperatedGroup()
            let og = duel::get_operated_group(d);
            // if og:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)~=res then return end
            if group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) as i32 != res {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local rg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
            let rg = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c)), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if #rg==0 then return end
            if rg.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
