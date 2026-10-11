//! Double Dust Tornado Twins
//! 砂塵の大ハリケーン
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 35479109;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return itself and Set Spell/Traps to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_SSET|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_SSET | TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToHand() and c:GetSequence()<5
    card::is_facedown(d, c) && card::is_able_to_hand(d, c, None) && card::get_sequence(d, c) < 5
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_SZONE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_SZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_SZONE,0,1,5,nil)
    let mut g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_SZONE, 0, 1, 5, false, NO_EX).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local rg=tg:Filter(aux.AND(Card.IsAbleToHand,Card.IsFacedown),nil)
    let mut rg = group::filter(d, &tg, &|d, c| card::is_able_to_hand(d, c, None) && card::is_facedown(d, c), NO_EX);
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // c:CancelToGrave()
    card::cancel_to_grave(d, c, true);
    // if not c:IsAbleToHand() then
    if !card::is_able_to_hand(d, c, None) {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
        // return
        return;
    }
    // if Duel.SendtoHand(rg:AddCard(c),nil,REASON_EFFECT)>0 then
    group::add_card(&mut rg, &CardSet::from_card(c));
    if duel::send_to_hand(d, rg, None, REASON_EFFECT, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_HAND):Match(Card.IsControler,nil,tp)
        let mut og = group::filter(d, &duel::get_operated_group(d), &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
        group::match_(d, &mut og, &|d, c| card::is_controler(d, c, tp), NO_EX);
        // local rt_ct=#og
        let rt_ct = og.len();
        // if rt_ct==0 then return end
        if rt_ct == 0 {
            return;
        }
        // local setg=Duel.GetMatchingGroup(Card.IsSSetable,tp,LOCATION_HAND,0,nil)
        let setg = duel::get_matching_group(d, Some(&|d, c| card::is_s_setable(d, c, false, None)), tp, LOCATION_HAND, 0, NO_EX);
        // if #setg==0 or #setg<rt_ct then return end
        if setg.is_empty() || setg.len() < rt_ct {
            return;
        }
        // if aux.SelectUnselectGroup(setg,e,tp,rt_ct,rt_ct,s.rescon,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        let valid = matches!(
            aux::select_unselect_group(d, &setg, Some(a.e), tp, Some(rt_ct), Some(rt_ct), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
        if valid && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local sg=aux.SelectUnselectGroup(setg,e,tp,rt_ct,rt_ct,s.rescon,1,tp,HINTMSG_SET)
            let sg = aux::select_unselect_group(d, &setg, Some(a.e), tp, Some(rt_ct), Some(rt_ct), Some(&rescon), 1, tp, Some(HINTMSG_SET as u64), None, None, false).unwrap_or_default();
            // Duel.SSet(tp,sg,tp,false)
            duel::s_set(d, tp, sg, Some(tp), false);
        }
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsSSetable,nil)==#sg
    //     and sg:FilterCount(aux.NOT(Card.IsType),nil,TYPE_FIELD)<=Duel.GetLocationCount(tp,LOCATION_SZONE)
    //     and sg:FilterCount(Card.IsType,nil,TYPE_FIELD)<=1
    let settable = group::filter_count(d, sg, &|d, c| card::is_s_setable(d, c, false, None), NO_EX) == sg.len();
    let non_field = group::filter_count(d, sg, &|d, c| !card::is_type(d, c, TYPE_FIELD), NO_EX) as i32;
    let res = settable
        && non_field <= duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0
        && group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_FIELD), NO_EX) <= 1;
    (res, false)
}
