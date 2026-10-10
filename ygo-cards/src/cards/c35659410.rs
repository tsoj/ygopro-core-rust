//! Vision with Eyes of Blue
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 35659410;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={CARD_BLUEEYES_W_DRAGON}
    listed_names: &[CARD_BLUEEYES_W_DRAGON],
    // s.listed_series={SET_BLUE_EYES}
    listed_series: &[SET_BLUE_EYES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if s.spcost(e,tp,eg,ep,ev,re,r,rp,0) and s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    let mut chk0 = a.clone();
    chk0.chk = Some(0);
    if spcost(d, &chk0) && sptg(d, &chk0) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // s.spcost(e,tp,eg,ep,ev,re,r,rp,1)
        let mut chk1 = a.clone();
        chk1.chk = Some(1);
        spcost(d, &chk1);
        // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
        sptg(d, &chk1);
        // e:SetOperation(s.spop)
        effect::set_operation(d, a.e, Some(operation(spop)));
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_BLUEEYES_W_DRAGON) and not c:IsPublic()
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && !card::is_public(d, c)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&spcostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, &g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
/// `s.sptgfilter(c,e,tp)`: `Card.IsCanBeSpecialSummoned` with `(e,0,tp,false,false)`
fn sptgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,nil,e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| sptgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,1,nil,e,0,tp,false,false)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| sptgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `aux.FaceupFilter(Card.IsAbleToHand)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.thspfilter(c,e,tp,thc)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, thc: CardId) -> bool {
    // return (thc:IsOriginalCode(CARD_BLUEEYES_W_DRAGON) or c:IsSetCard(SET_BLUE_EYES)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_original_code(d, thc, &[CARD_BLUEEYES_W_DRAGON]) || card::is_set_card(d, c, &[SET_BLUE_EYES as u16]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local op=2
    let mut op = 2;
    // if not tc:IsOriginalCode(CARD_BLUEEYES_W_DRAGON) then op=3 end
    if !card::is_original_code(d, tc, &[CARD_BLUEEYES_W_DRAGON]) {
        op = 3;
    }
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_HAND)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_HAND,0,1,nil,e,tp,tc)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,op)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, tc)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, op))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,tc)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, tc)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
