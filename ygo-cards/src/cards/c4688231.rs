//! Metalfoes Mithrilium
use super::prelude::*;

pub const ID: u32 = 4688231;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_METALFOES],
    material_setcode: &[SET_METALFOES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_METALFOES),aux.FilterBoolFunctionEx(Card.IsType,TYPE_PENDULUM))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_METALFOES as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), st, Some(tp)))),
        ],
    );
    // return
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.rettg)
    effect::set_target(d, e2, Some(cost_target(rettg)));
    // e2:SetOperation(s.retop)
    effect::set_operation(d, e2, Some(operation(retop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.retfilter1(c)`
fn retfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_METALFOES) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_METALFOES as u16]) && card::is_able_to_deck(d, c)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.retfilter1,tp,LOCATION_GRAVE,0,2,nil)
    //   and Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&retfilter1), a.tp, LOCATION_GRAVE, 0, 2, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.retfilter1,tp,LOCATION_GRAVE,0,2,2,nil)
    let g1 = duel::select_target(d, a.tp, Some(&retfilter1), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,#g1,0,0)
    let n1 = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1), n1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g2, 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local g1=g:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let g1 = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if Duel.SendtoDeck(g1,nil,SEQ_DECKTOP,REASON_EFFECT)~=0 then
    if duel::send_to_deck(d, g1, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, a.tp);
        }
        // local g2=g:Filter(Card.IsLocation,nil,LOCATION_ONFIELD)
        let g2 = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_ONFIELD), NO_EX);
        // Duel.SendtoHand(g2,nil,REASON_EFFECT)
        duel::send_to_hand(d, g2, None, REASON_EFFECT, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)==0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 == 0 {
        return false;
    }
    // return c:IsSetCard(SET_METALFOES) and c:IsType(TYPE_PENDULUM) and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_METALFOES as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_GRAVE end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_GRAVE;
    }
    // if chk==0 then return loc~=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,loc,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return loc != 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, loc as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_GRAVE end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_GRAVE;
    }
    // if loc==0 then return end
    if loc == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,loc,0,1,1,nil,e,tp)
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
