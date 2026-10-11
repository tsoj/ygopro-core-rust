//! Vernusylph and the Flower Buds
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 63708033;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return cards to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32 | TIMING_MAIN_END as u32));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}

/// `s.ctfilter(c)`: `c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsSummonLocation(LOCATION_GRAVE) and c:IsSpecialSummoned()`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_summon_location(d, c, LOCATION_GRAVE) && card::is_special_summoned(d, c)
}

/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroup(s.ctfilter,tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetCode)
    let mg = duel::get_matching_group(d, Some(&ctfilter), tp, LOCATION_MZONE, 0, NO_EX);
    let ct = group::get_class_count(d, &mg, &|d, c| card::get_code(d, c)[0] as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)),
        tp,
        0,
        LOCATION_ONFIELD,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g*2,PLAYER_ALL,LOCATION_MZONE)
    let n = g.len() as u8 * 2;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}

/// `s.thfilter(c)`: `c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_able_to_hand(d, c, None)
}

/// `s.thtgfilter(c,e,tp)`: `c:IsRelateToEffect(e) and c:IsFaceup() and c:IsControler(1-tp) and c:IsAbleToHand()`
fn thtgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && card::is_controler(d, c, 1 - tp) && card::is_able_to_hand(d, c, None)
}

/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Clone()
    let tg_info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let mut tg: CardSet = tg_info[0].as_group().cloned().nil();
    let n = tg.len() as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_MZONE,0,#tg,#tg,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_MZONE, 0, n, n, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g,true)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    //     and #tg:Match(s.thtgfilter,nil,e,tp)>0 then
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0
        && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
    {
        // Group:Match filters the group in place
        group::match_(d, &mut tg, &|d, c| thtgfilter(d, c, a.e, tp), NO_EX);
        if !tg.is_empty() {
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
        }
    }
}
