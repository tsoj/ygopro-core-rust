//! Birth of the Prominence Flame
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 5439384;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 2 FIRE monsters (1 Tuner and 1 non-Tuner) with 200 DEF in your GY; banish both monsters, and if you do, Special Summon 1 FIRE Synchro Monster from your Extra Deck whose Level equals the total Levels of those monsters. You can only activate 1 "Birth of the Prominence Flame" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsDefense(200) and c:HasLevel() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_defense(d, c, &[200])
        && card::has_level(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:IsExists(Card.IsType,1,nil,TYPE_TUNER) and sg:IsExists(aux.NOT(Card.IsType),1,nil,TYPE_TUNER)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg:GetSum(Card.GetLevel),sg)
    let has_tuner = group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), 1, NO_EX);
    let has_non_tuner = group::is_exists(d, sg, &|d, c| !card::is_type(d, c, TYPE_TUNER), 1, NO_EX);
    let res = has_tuner && has_non_tuner && {
        let lv = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, lv, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    };
    (res, false)
}
/// `s.spfilter(c,e,tp,lv,sg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, sg: Option<&CardSet>) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsSynchroMonster() and c:IsLevel(lv) and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_synchro_monster(d, c)
        && card::is_level(d, c, &[lv as u32])
        && match sg {
            Some(sg) => duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(c), None).0 > 0,
            None => duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0,
        }
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_target_group(d, Some(&tgfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg==2 and Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)==2 then
    if tg.len() == 2 && duel::remove(d, tg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tg:GetSum(Card.GetLevel))
        let lv = group::get_sum(d, &tg, &|d, c| card::get_level(d, c) as i64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
