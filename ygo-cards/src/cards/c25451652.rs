//! Darklord Morningstar
use super::prelude::*;

pub const ID: u32 = 25451652;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(lua_fn(tgcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // discard deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DECKDES+CATEGORY_RECOVER)
    effect::set_category(d, e4, CATEGORY_DECKDES + CATEGORY_RECOVER);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DARKLORD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.ctfilter,tp,0,LOCATION_MZONE,1,nil)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetMatchingGroup(s.ctfilter,tp,0,LOCATION_MZONE,nil)
    let g1 = duel::get_matching_group(d, Some(&ctfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local ct=5
    let mut ct: i64 = 5;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // ct=math.min(ct,#g1,Duel.GetLocationCount(tp,LOCATION_MZONE))
    ct = ct.min(g1.len() as i64).min(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64);
    // if ct>0 and #g2>0 then
    if ct > 0 && !g2.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local tg=g2:Select(tp,1,ct,nil)
        let tg = group::select(d, &g2, tp, 1, ct as u16, false, NO_EX).nil();
        // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARKLORD)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARKLORD as u16])
}
/// `s.tgcon(e)`: `Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())`
fn tgcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as u32;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,ct) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, ct);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, ct as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,ct*1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, (ct * 1000) as i32);
    true
}
/// `s.ctfilter2(c)`
fn ctfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsSetCard(SET_DARKLORD)
    card::is_location(d, c, LOCATION_GRAVE) && card::is_set_card(d, c, &[SET_DARKLORD as u16])
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct1=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct1 = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as u16;
    // if ct1>0 then
    if ct1 > 0 {
        // if Duel.DiscardDeck(tp,ct1,REASON_EFFECT)~=0 then
        if duel::discard_deck(d, tp, ct1, REASON_EFFECT) != 0 {
            // local og=Duel.GetOperatedGroup()
            let og = duel::get_operated_group(d);
            // local ct2=og:FilterCount(s.ctfilter2,nil)
            let ct2 = group::filter_count(d, &og, &ctfilter2, NO_EX) as i64;
            // if ct2>0 then
            if ct2 > 0 {
                // Duel.Recover(tp,ct2*500,REASON_EFFECT)
                duel::recover(d, tp, ct2 * 500, REASON_EFFECT, false, None);
            }
        }
    }
}
