//! Materiactor Meltdown
//! 原質の炉心溶融
use super::prelude::*;

pub const ID: u32 = 38669664;

// s.listed_series={SET_MATERIACTOR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Excavate the top 6 cards of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.exctg)
    effect::set_target(d, e1, Some(cost_target(exctg)));
    // e1:SetOperation(s.excop)
    effect::set_operation(d, e1, Some(operation(excop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach the top card of your Deck to 1 "Materiactor" Xyz Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==tp and re:IsMonsterEffect() and re:IsActiveType(TYPE_XYZ) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRank(3) and c:IsType(TYPE_XYZ) and c:IsFaceup() and c:GetOverlayGroup():IsExists(Card.IsAbleToHand,1,nil)
    if !(card::is_rank(d, c, &[3]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)) {
        return false;
    }
    let ov = card::get_overlay_group(d, c);
    group::is_exists(d, &ov, &|d, x| card::is_able_to_hand(d, x, None), 1, NO_EX)
}
/// `s.exctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=6 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= 6;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_OVERLAY)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_OVERLAY) as i32);
    true
}
/// `s.excop(e,tp,eg,ep,ev,re,r,rp)`
fn excop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<6 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < 6 {
        return;
    }
    // local ct=6
    let mut ct: i64 = 6;
    // Duel.ConfirmDecktop(tp,6)
    duel::confirm_decktop(d, tp, 6);
    // local g=Duel.GetDecktopGroup(tp,6)
    let g = duel::get_decktop_group(d, tp, 6);
    // if g:IsExists(s.deckthfilter,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if group::is_exists(d, &g, &|d, c| deckthfilter(d, c), 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:FilterSelect(tp,s.deckthfilter,1,1,nil)
        let sg = group::filter_select(d, &g, tp, &|d, c| deckthfilter(d, c), 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.DisableShuffleCheck()
            duel::disable_shuffle_check(d, true);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // ct=ct-1
            ct -= 1;
        }
    }
    // Duel.SortDecktop(tp,tp,ct)
    duel::sort_decktop(d, tp, tp, ct as u16);
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_MZONE,0,nil)
    let xyzg = duel::get_matching_group(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if #xyzg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if !xyzg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // local og=Duel.GetOverlayGroup(tp,1,0,xyzg)
        let og = duel::get_overlay_group(d, tp, 1, 0, Some(&xyzg));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=og:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let g = group::filter_select(d, &og, tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // local owner=g:GetFirst():GetControler()
            let first = group::get_first(d, &g).nil();
            let owner = card::get_controler(d, first);
            // Duel.ConfirmCards(1-owner,g)
            duel::confirm_cards(d, 1 - owner, g);
            // Duel.ShuffleHand(owner)
            duel::shuffle_hand(d, owner);
        }
    }
}
/// `s.attachfilter(c)`
fn attachfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==tp and re:IsMonsterEffect() and re:IsActiveType(TYPE_XYZ) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && effect::is_active_type(d, re, TYPE_XYZ)
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //     and Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0
            && duel::is_existing_matching_card(d, Some(&attachfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
    // local xyzc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&attachfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let xyzc = group::get_first(d, &sel);
    // if xyzc then
    if let Some(xyzc) = xyzc {
        // Duel.HintSelection(xyzc)
        duel::hint_selection(d, xyzc, true);
        // local g=Duel.GetDecktopGroup(tp,1)
        let g = duel::get_decktop_group(d, tp, 1);
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Overlay(xyzc,g)
        duel::overlay(d, xyzc, g, false);
    }
}
