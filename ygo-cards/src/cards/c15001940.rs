//! Foie Glasya de Nouvelles
use super::prelude::*;

pub const ID: u32 = 15001940;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RECIPE, SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Register that it was Special Summoned by the effect of a "Nouvelles" monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2)) end)
    effect::set_operation(d, e0, Some(operation(reg_op)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Shuffle cards from the GY into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 5 or 6 "Nouvelles" Ritual Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e3, EVENT_BECOME_TARGET);
    // e3:SetCondition(function(_,_,eg) return eg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.regcon(e,tp,eg,ep,ev,re,r,rp)
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsMonsterEffect() and re:IsCardSetcode(SET_NOUVELLES)
    match a.re {
        Some(re) => effect::is_monster_effect(d, re) && effect::is_card_setcode(d, re, &[SET_NOUVELLES as u16]),
        None => false,
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) e:GetHandler():RegisterFlagEffect(...) end
fn reg_op(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(
        d,
        h,
        ID,
        RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE),
        EFFECT_FLAG_CLIENT_HINT,
        1,
        0,
        aux::stringid(ID, 2),
    );
}
// function(_,_,eg) return eg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) end
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,3,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 3, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, &sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReleasableByEffect() and (s.selfnouvfilter(c,tp) or c:IsAttackPos())
    card::is_releasable_by_effect(d, c, None, None) && (selfnouvfilter(d, c, tp) || card::is_attack_pos(d, c))
}
// function s.selfnouvfilter(c,tp)
fn selfnouvfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsSetCard(SET_NOUVELLES)
    card::get_controler(d, c) == tp && card::is_set_card(d, c, &[SET_NOUVELLES as u16])
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOUVELLES) and c:IsRitualMonster() and c:IsLevel(5,6)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,true)
    card::is_set_card(d, c, &[SET_NOUVELLES as u16])
        && card::is_ritual_monster(d, c)
        && card::is_level(d, c, &[5, 6])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:IsExists(s.atkposchk,1,nil,sg,tp)
    let zone = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0;
    let res = zone && group::is_exists(d, sg, &|d, c| atkposchk(d, c, sg, tp), 1, NO_EX);
    (res, false)
}
// function s.atkposchk(c,sg,tp)
fn atkposchk(d: &mut Duel, c: CardId, sg: &CardSet, tp: u8) -> bool {
    // return c:IsAttackPos() and sg:IsExists(s.selfnouvfilter,1,c,tp)
    card::is_attack_pos(d, c) && group::is_exists(d, sg, &|d, x| selfnouvfilter(d, x, tp), 1, c.into())
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>=2 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, false)
        .unwrap_or_default();
    // if Duel.Release(rg,REASON_EFFECT)==2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::release(d, &rg, REASON_EFFECT, None) == 2 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,true,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, true, POS_FACEUP as u8, None);
        }
    }
}
