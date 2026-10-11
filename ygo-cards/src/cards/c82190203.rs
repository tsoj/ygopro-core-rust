//! Wings of Light
use super::prelude::*;

pub const ID: u32 = 82190203;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ZARC],
    listed_series: &[SET_SUPREME_KING_DRAGON, SET_SUPREME_KING_GATE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND|CATEGORY_SEARCH|CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c,e,tp,zarc_chk)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zarc_chk: bool) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsMonster() and c:IsSetCard({SET_SUPREME_KING_DRAGON,SET_SUPREME_KING_GATE})
    //     and (c:IsAbleToHand() or (zarc_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_type(d, c, TYPE_PENDULUM)
        && card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16, SET_SUPREME_KING_GATE as u16])
        && (card::is_able_to_hand(d, c, None) || (zarc_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// zarc_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ZARC),tp,LOCATION_ONFIELD,0,1,nil)
fn zarc_check(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local zarc_chk=...
    let zarc_chk = zarc_check(d, tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,zarc_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, zarc_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local zarc_chk=...
    let zarc_chk = zarc_check(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,zarc_chk):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, zarc_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // aux.ToHandOrElse(sc,tp,
    //     function(sc)
    //         return zarc_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     end,
    //     function(sc)
    //         return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
    //     end,
    //     aux.Stringid(id,1)
    // )
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, c| zarc_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
        Some(&|d, g| duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 1)),
    );
}
