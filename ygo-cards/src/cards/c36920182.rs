//! Realm Resonance
use super::prelude::*;

pub const ID: u32 = 36920182;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_VISAS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster from the GY or return monsters and summon from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return s.statsfilter(c) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    statsfilter(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.statsfilter(c)`
fn statsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttack(1500) and c:IsDefense(2100)
    card::is_attack(d, c, &[1500]) && card::is_defense(d, c, &[2100])
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToDeck()
    //   and (c:IsCode(CARD_VISAS_STARFROST) or s.statsfilter(c))
    card::is_faceup(d, c) && card::is_able_to_deck(d, c) && (card::is_code(d, c, &[CARD_VISAS_STARFROST]) || statsfilter(d, c))
}
/// `s.synchfilter(c,e,tp,zonecheck)`
fn synchfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zonecheck: bool) -> bool {
    // return c:IsSetCard(SET_VISAS) and c:IsType(TYPE_SYNCHRO)
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //   and (zonecheck or Duel.GetLocationCountFromEx(tp,tp,nil,c)>0)
    card::is_set_card(d, c, &[SET_VISAS as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
        && (zonecheck || duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,sg,TYPE_SYNCHRO)>0 and sg:IsExists(s.cfilter,1,nil,sg)
    let res = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Type(TYPE_SYNCHRO), None).0 > 0
        && group::is_exists(d, sg, &|d, c| cfilter(d, c, sg), 1, NO_EX);
    (res, false)
}
/// `s.cfilter(c,sg)`
fn cfilter(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(CARD_VISAS_STARFROST) and sg:FilterCount(s.statsfilter,c)==4
    card::is_code(d, c, &[CARD_VISAS_STARFROST]) && group::filter_count(d, sg, &|d, x| statsfilter(d, x), c.into()) == 4
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| gyspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local b2=#g>=5 and Duel.IsExistingMatchingCard(s.synchfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,true)
    //   and aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,0)
    let b2 = g.len() >= 5
        && duel::is_existing_matching_card(d, Some(&|d, c| synchfilter(d, c, e, tp, true)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(5), Some(5), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,1)},
    //   {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.spop)
        effect::set_operation(d, e, Some(operation(spop)));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.synchop)
        effect::set_operation(d, e, Some(operation(synchop)));
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,5,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 5, tp, (LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| gyspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.synchop(e,tp,eg,ep,ev,re,r,rp)`
fn synchop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(tdfilter)), tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if #g<5 then return end
    if g.len() < 5 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,1,tp,HINTMSG_TODECK)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(5), Some(5), Some(&rescon), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // if #rg~=5 then return end
    if rg.len() != 5 {
        return;
    }
    // Duel.HintSelection(rg,true)
    duel::hint_selection(d, rg.clone(), true);
    // if Duel.SendtoDeck(rg,tp,SEQ_DECKSHUFFLE,REASON_EFFECT)==5
    //   and Duel.IsExistingMatchingCard(s.synchfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,false) then
    if duel::send_to_deck(d, rg, Some(tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 5
        && duel::is_existing_matching_card(d, Some(&|d, c| synchfilter(d, c, e, tp, false)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.synchfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,false):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| synchfilter(d, c, e, tp, false)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg);
        // if sc then
        if let Some(sc) = sc {
            // Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO as u32, tp, tp, false, false, POS_FACEUP as u8, None);
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
