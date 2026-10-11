//! Wattkyuki
//! エレキュウキ
use super::prelude::*;

pub const ID: u32 = 67752972;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WATT], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_WATT),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_THUNDER),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_WATT as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_THUNDER, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle monsters from the GY and field into the Deck and Special Summon 1 "Watt" Synchro Monster from your Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spsynccon)
    effect::set_condition(d, e2, Some(condition(spsynccon)));
    // e2:SetTarget(s.spsynctg)
    effect::set_target(d, e2, Some(cost_target(spsynctg)));
    // e2:SetOperation(s.spsyncop)
    effect::set_operation(d, e2, Some(operation(spsyncop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spsynccon(e,tp,eg,ep,ev,re,r,rp)`
fn spsynccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.GetAttackTarget()==nil
    a.ep == 1 - a.tp && duel::get_attack_target(d).is_none()
}
/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WATT) and c:IsType(TYPE_TUNER) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_WATT as u16]) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_deck(d, c)
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_THUNDER) and not c:IsType(TYPE_TUNER) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_THUNDER) && !card::is_type(d, c, TYPE_TUNER) && card::is_able_to_deck(d, c)
}
/// `s.spfilter(c,e,tp,matg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, matg: Option<&CardSet>) -> bool {
    // return c:IsSetCard(SET_WATT) and c:IsType(TYPE_SYNCHRO) and not c:IsCode(id)
    //     and Duel.GetLocationCountFromEx(tp,tp,matg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WATT as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && !card::is_code(d, c, &[ID])
        && duel::get_location_count_from_ex(d, tp, Some(tp), matg.map_or(NO_EX, |g| g.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_MZONE)==1
    //     and sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg)
    let ok = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX) == 1
        && group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) == 1
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (ok, false)
}
/// `g1+g2`
fn union(a: &CardSet, b: &CardSet) -> CardSet {
    let mut u = a.clone();
    group::add_card(&mut u, b);
    u
}
/// `s.spsynctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spsynctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_GRAVE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&gyfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.mzfilter,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&mzfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g1>0 and #g2>0 and aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return !g1.is_empty()
            && !g2.is_empty()
            && matches!(
                aux::select_unselect_group(d, &union(&g1, &g2), Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spsyncop(e,tp,eg,ep,ev,re,r,rp)`
fn spsyncop(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_GRAVE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&gyfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.mzfilter,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&mzfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g1<1 or #g2<1 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,1,tp,HINTMSG_TODECK)
    let rg = aux::select_unselect_group(d, &union(&g1, &g2), Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // if #rg~=2 then return end
    if rg.len() != 2 {
        return;
    }
    // Duel.HintSelection(rg,true)
    duel::hint_selection(d, rg.clone(), true);
    // if Duel.SendtoDeck(rg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and Duel.GetOperatedGroup():IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then
    if duel::send_to_deck(d, rg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        let operated = duel::get_operated_group(d);
        if group::is_exists(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,nil):GetFirst()
            let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if sc then
            if let Some(sc) = group::get_first(d, &sel) {
                // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
