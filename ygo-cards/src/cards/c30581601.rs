//! Yummy★Snatchy
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 30581601;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Level 4 or lower LIGHT Beast monster
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
    // Place 1 "Yummy" Field Spell from your hand or Deck face-up on your field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Immediately after this effect resolves, Synchro Summon 1 Synchro Monster, using monsters you control as material, including a "Yummy" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER) as u32));
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase() or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.PayLP(100))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(100))));
    // e2:SetTarget(s.synchtg)
    effect::set_target(d, e2, Some(cost_target(synchtg)));
    // e2:SetOperation(s.synchop)
    effect::set_operation(d, e2, Some(operation(synchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_LIGHT,lc,sumtype,tp) and c:IsRace(RACE_BEAST,lc,sumtype,tp)
    card::is_level_below(d, c, 4) && card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, lc, sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_BEAST, lc, sumtype, Some(tp))
}
/// `function(e,tp) return Duel.IsMainPhase() or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) || (duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None))
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsFieldSpell() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_field_spell(d, c) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
        // if fc then
        if let Some(fc) = fc {
            // Duel.SendtoGrave(fc,REASON_RULE)
            duel::send_to_grave(d, CardSet::from_card(fc), REASON_RULE, None, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
        }
        // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
    // You cannot Link Summon Link-3 or higher Link Monsters for the rest of this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(function(e,c,tp,sumtyp,sumpos) return c:IsLinkAbove(3) and c:IsLinkMonster() and (sumtyp&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK end)
    effect::set_target(d, e1, Some(lua_fn(link_target)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e,c,tp,sumtyp,sumpos) return c:IsLinkAbove(3) and c:IsLinkMonster() and (sumtyp&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK end`
fn link_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtyp = a.int(3) as u64;
    card::is_link_above(d, c, 3) && card::is_link_monster(d, c) && (sumtyp & SUMMON_TYPE_LINK as u64) == SUMMON_TYPE_LINK as u64
}
/// `s.synchrocheck(tp,sg,sc)`
fn synchrocheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_YUMMY)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_set_card(d, c, &[SET_YUMMY as u16]), 1, NO_EX)
}
/// `s.synchtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn synchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // Synchro.CheckAdditional=s.synchrocheck
        synchro::set_check_additional(d, Some(lua_fn(synchrocheck)));
        // local res=Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.synchop(e,tp,eg,ep,ev,re,r,rp)`
fn synchop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Synchro.CheckAdditional=s.synchrocheck
    synchro::set_check_additional(d, Some(lua_fn(synchrocheck)));
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // Duel.SynchroSummon(tp,sc)
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    } else {
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
    }
}
