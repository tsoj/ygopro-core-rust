//! Visas Samsara
use super::prelude::*;

pub const ID: u32 = 14391625;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_VISAS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Visas Starfrost"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_VISAS_STARFROST)
    effect::set_value(d, e1, CARD_VISAS_STARFROST as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can be treated as a non-tuner for a Synchro Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e3, EFFECT_NONTUNER);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tdfilter(c,tp)`
fn tdfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VISAS) and c:IsMonster() and c:IsAbleToDeck() and Duel.GetMZoneCount(tp,c)>0
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_VISAS as u16])
        && card::is_monster(d, c)
        && card::is_able_to_deck(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=LOCATION_MZONE|LOCATION_REMOVED|LOCATION_GRAVE
    let loc = LOCATION_MZONE | LOCATION_REMOVED | LOCATION_GRAVE;
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(loc) and s.tdfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, loc) && tdfilter(d, chkc, tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.tdfilter,tp,loc,0,1,nil,tp) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, cc| tdfilter(d, cc, tp)), tp, loc, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.tdfilter,tp,loc,0,nil,tp)
    let ct = duel::get_matching_group_count(d, Some(&|d, cc| tdfilter(d, cc, tp)), tp, loc, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,loc,0,1,ct,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, cc| tdfilter(d, cc, tp)), tp, loc, 0, 1, ct as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.as_ref().map_or(0, |g| g.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.visasfilter(c)`
fn visasfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VISAS) and c:IsMonster()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VISAS as u16]) && card::is_monster(d, c)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Match(s.visasfilter,nil)
    let mut g = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut g, &|d, c| visasfilter(d, c), NO_EX);
    // if #g==0 or Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let mut og = duel::get_operated_group(d);
    group::match_(d, &mut og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // c:UpdateAttack(400*og:GetClassCount(Card.GetCode))
        let cnt = group::get_class_count(d, &og, &|d, cc| card::get_code(d, cc)[0] as i64) as i64;
        card::update_attack(d, c, 400 * cnt, None, None, None);
    }
}
