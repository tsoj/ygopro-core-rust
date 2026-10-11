//! Gordian Slicer
use super::prelude::*;

pub const ID: u32 = 66236707;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish cards from your hand and/or from your Extra Deck face-down and return the targeted cards to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`: `Duel.IsMainPhase() and not Duel.CheckPhaseActivity()`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && !duel::check_phase_activity(d)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() and chkc:IsFaceup() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None) && card::is_faceup(d, chkc) && chkc != c;
    }
    // local ct1=Duel.GetMatchingGroupCount(Card.IsAbleToRemove,tp,LOCATION_HAND,0,c,tp,POS_FACEDOWN)
    let tp = a.tp;
    let ct1 = duel::get_matching_group_count(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_HAND, 0, c.into());
    // local ct2=Duel.GetMatchingGroupCount(Card.IsAbleToRemove,tp,LOCATION_EXTRA,0,nil,tp,POS_FACEDOWN)
    let ct2 = duel::get_matching_group_count(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return (ct1>=1 or ct2>=6) and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return (ct1 >= 1 || ct2 >= 6)
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local ct=ct1+(ct2//6)
    let ct = ct1 + ct2 / 6;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,c)
    let g = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // local ct=#tg
    let ct = tg.len();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_HAND,0,nil,tp,POS_FACEDOWN)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_EXTRA,0,nil,tp,POS_FACEDOWN)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if (#g1+#g2//6)<ct then return end
    if g1.len() + g2.len() / 6 < ct {
        return;
    }
    // local rg=g1+g2
    let rg = group::union(&g1, &g2);
    // local sg=aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon(ct),1,tp,HINTMSG_REMOVE,s.rescon(ct),s.rescon(ct))
    let rc = move |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>| -> (bool, bool) {
        rescon(d, sg, ct, e, tp, mg, c)
    };
    let rs: aux::Rescon = &rc;
    let sg = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(1), Some(rg.len()), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), Some(rs), Some(rs), false).unwrap_or_default();
    // if Duel.Remove(sg,POS_FACEDOWN,REASON_EFFECT)>0 then
    if duel::remove(d, sg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
/// `s.rescon(ct)` returned function: `(sg,e,tp,mg)` giving `(valid, stop)`
fn rescon(d: &mut Duel, sg: &CardSet, ct: usize, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // (the Lua closure `s.rescon(ct)`: the arguments after `sg` are not used)
    // local count=sg:FilterCount(Card.IsLocation,nil,LOCATION_HAND)
    let count = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX) as i64;
    // local remaining=ct-count
    let remaining = ct as i64 - count;
    // if remaining<0 then return false,true end
    if remaining < 0 {
        return (false, true);
    }
    // local extracnt=sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
    let extracnt = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX) as i64;
    // local extra_needed=remaining*6
    let extra_needed = remaining * 6;
    // if extracnt>extra_needed then return false,true end
    if extracnt > extra_needed {
        return (false, true);
    }
    // return extracnt==extra_needed,extracnt>extra_needed
    (extracnt == extra_needed, extracnt > extra_needed)
}
