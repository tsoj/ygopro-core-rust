//! Synchro Emergency
//! シンクロ・エマージェンシー
use super::prelude::*;

pub const ID: u32 = 49415281;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYNCHRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If only your opponent controls a monster, you can activate this card from your hand
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_HAND);
    // e0:SetCondition(function(e) local tp=e:GetHandlerPlayer() return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end)
    effect::set_condition(d, e0, Some(condition(cond0)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 monster from your hand (but negate its effects), then if your opponent controls a monster Special Summoned from the Extra Deck, you can Special Summon 1 "Synchron" monster from your Deck. Until the end of the next turn after this card resolves, you cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E|TIMING_CHAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_CHAIN_END));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; immediately after this effect resolves, Synchro Summon 1 Synchro Monster
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
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.synctg)
    effect::set_target(d, e2, Some(cost_target(synctg)));
    // e2:SetOperation(s.syncop)
    effect::set_operation(d, e2, Some(operation(syncop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E|TIMING_CHAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_CHAIN_END));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) local tp=e:GetHandlerPlayer() return ... end` of e0
fn cond0(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.handspfilter(c,e,tp)`
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.handspfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| handspfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspfilter(c,e,tp)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SYNCHRON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SYNCHRON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.handspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, cc| handspfilter(d, cc, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if let Some(sc) = sc {
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // --Negate its effects
                // sc:NegateEffects(c)
                card::negate_effects(d, sc, c, None, false, None);
                // if Duel.SpecialSummonComplete()>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
                if duel::special_summon_complete(d) > 0
                    && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    // and Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,0,LOCATION_MZONE,1,nil,LOCATION_EXTRA)
                    && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_summon_location(d, cc, LOCATION_EXTRA)), tp, 0, LOCATION_MZONE, 1, NO_EX)
                    // and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
                    && duel::is_existing_matching_card(d, Some(&|d, cc| deckspfilter(d, cc, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
                    // and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
                {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
                    let g = duel::select_matching_card(d, tp, Some(&|d, cc| deckspfilter(d, cc, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                    // if #g>0 then
                    if !g.is_empty() {
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    }
                }
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // --Until the end of the next turn after this card resolves, you cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSynchroMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_synchro_monster(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.synctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn synctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.syncop(e,tp,eg,ep,ev,re,r,rp)`
fn syncop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local synchro=Duel.SelectMatchingCard(tp,Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let synchro = group::get_first(d, &sel);
    // if synchro then
    if let Some(synchro) = synchro {
        // Duel.SynchroSummon(tp,synchro,nil)
        duel::synchro_summon(d, tp, synchro, None, None, 0, 0);
    }
}
