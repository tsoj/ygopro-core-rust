//! K9-X "Werewolf"
use super::prelude::*;

pub const ID: u32 = 90303227;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 9 monsters
    // Xyz.AddProcedure(c,nil,9,2)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, None, None, false, None);
    // Can attack a number of times each Battle Phase, up to the number of materials attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(function(e,c) return math.max(0,e:GetHandler():GetOverlayCount()-1) end)
    effect::set_value_fn(d, e1, effect_value(extra_attack_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate this effect depending on whose turn it is
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn extra_attack_value(d: &mut Duel, e: EffectId) -> i64 {
    // math.max(0,e:GetHandler():GetOverlayCount()-1)
    let h = effect::get_handler(d, e).nil();
    (card::get_overlay_count(d, h) as i64 - 1).max(0)
}
fn cond2(_d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp
    a.rp == 1 - a.tp
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=Duel.IsTurnPlayer(tp) and LOCATION_ONFIELD|LOCATION_GRAVE or LOCATION_HAND
    let loc = if duel::is_turn_player(d, tp) { LOCATION_ONFIELD | LOCATION_GRAVE } else { LOCATION_HAND };
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,loc,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, loc, 1, NO_EX);
    }
    // e:SetLabel(loc)
    effect::set_label(d, a.e, &[loc as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, loc as i32);
    true
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_ONFIELD)<=1
    //   and sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)<=1
    let res = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_ONFIELD), NO_EX) <= 1
        && group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) <= 1;
    (res, false)
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local loc=e:GetLabel()
    let loc = effect::get_label(d, a.e) as u32;
    // local g=Duel.GetFieldGroup(tp,0,loc)
    let g = duel::get_field_group(d, tp, 0, loc);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    if loc == LOCATION_HAND {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, &g);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local tc=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil)
        let tc = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX);
        // if tc then
        if let Some(tc) = tc {
            // aux.RemoveUntil(tc,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,function(rg,e,tp) Duel.SendtoHand(tc,nil,REASON_EFFECT) end)
            let tc2 = tc.clone();
            let oper = std::sync::Arc::new(move |d: &mut Duel, _rg: &CardSet, _a: &EffArgs| {
                duel::send_to_hand(d, tc2.clone(), None, REASON_EFFECT, None);
                true
            });
            aux::remove_until(d, tc, Some(POS_FACEUP as u8), REASON_EFFECT | REASON_TEMPORARY, PHASE_END, ID, a.e, tp, Some(oper), None, None, None, None, None);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    } else {
        // local rg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_REMOVE)
        let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
        // if #rg==0 then return end
        if rg.is_empty() {
            return;
        }
        // Duel.HintSelection(rg)
        duel::hint_selection(d, rg.clone(), true);
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
