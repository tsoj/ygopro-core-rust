//! R-Genex Turing
use super::prelude::*;

pub const ID: u32 = 61052897;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GENEX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be treated as a Level 1 or 3 monster for the Synchro Summon of a "Genex" Synchro monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e2, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.synop)
    effect::set_operation(d, e2, Some(lua_fn(|d, a| synop(d, &a))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GENEX),tp,LOCATION_MZONE,0,1,nil)
    let tp = a.tp;
    duel::is_turn_player(d, 1 - tp)
        && duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GENEX as u16]))),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.scfilter(c)
fn scfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsSynchroSummonable()
    card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_synchro_summonable(d, c, None, None, 0, 0)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.scfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&scfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if group::get_count(&g) == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SynchroSummon(tp,sc,nil)
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    }
}
// function s.synop(e,tg,ntg,sg,lv,sc,tp)
fn synop(d: &mut Duel, a: &Args) -> Value {
    let e = a.e(0);
    let sg = a.g(3);
    let lv = a.int(4);
    let sc = a.c(5);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local sum=(sg-c):GetSum(Card.GetSynchroLevel,sc)
    let mut rest = sg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(c));
    let sum = group::get_sum(d, &rest, &|d, x| card::get_synchro_level(d, x, sc) as i64);
    // if sum+c:GetSynchroLevel(sc)==lv then return true,true end
    if sum + card::get_synchro_level(d, c, sc) as i64 == lv {
        return Value::Multi(vec![Value::Bool(true), Value::Bool(true)]);
    }
    // return sc:IsSetCard(SET_GENEX) and ((sum+1==lv) or (sum+3==lv)),true
    Value::Multi(vec![
        Value::Bool(card::is_set_card(d, sc, &[SET_GENEX as u16]) && (sum + 1 == lv || sum + 3 == lv)),
        Value::Bool(true),
    ])
}
