//! Ultimate Crystal Rainbow Dragon Overdrive
//! Scripted by Hatter
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 84544192;

// s.listed_series={SET_ULTIMATE_CRYSTAL,SET_CRYSTAL_BEAST}
// s.material_setcode={SET_ULTIMATE_CRYSTAL,SET_CRYSTAL_BEAST}
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ULTIMATE_CRYSTAL, SET_CRYSTAL_BEAST],
    material_setcode: &[SET_ULTIMATE_CRYSTAL, SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 "Ultimate Crystal" Monster + 7 "Crystal Beast" Monsters
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_CRYSTAL_BEAST),7,7,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ULTIMATE_CRYSTAL))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_CRYSTAL_BEAST as u16], Some(fc), st, Some(tp)))),
        7,
        7,
        vec![fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ULTIMATE_CRYSTAL as u16], Some(fc), st, Some(tp))))],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.contactlim,s.contactcon)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(|d, tp| contactfil(d, tp))),
            material_operation: Some(Arc::new(|d, g, _tp, _c| contactop(d, g))),
            sumcon: Some(Some(lua_fn(contactlim))),
            condition: Some(Arc::new(|d, tp| contactcon(d, tp))),
            ..Default::default()
        },
    );
    // Register "Ultimate Crystal" Special Summon
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(s.regop)
        effect::set_operation(d, ge1, Some(operation(regop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Cannot be Special Summoned by other ways
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Gains 700 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(7000)
    effect::set_value(d, e1, 7000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle as many cards into the Deck as possible and Special Summon "Crystal Beast" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetCondition(function(e) return e:GetHandler():GetBattledGroupCount()==0 end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_battled_group_count(d, h) == 0
    })));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // local loc=Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and LOCATION_MZONE or LOCATION_MZONE|LOCATION_GRAVE
    let loc = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        LOCATION_MZONE
    } else {
        LOCATION_MZONE | LOCATION_GRAVE
    };
    // return Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,loc,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, loc, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet) {
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.contactlim(e)`
fn contactlim(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_EXTRA)
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_location(d, h, LOCATION_EXTRA)
}
/// `s.contactcon(tp)`
fn contactcon(d: &mut Duel, tp: u8) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0
    duel::get_flag_effect(d, tp, ID) > 0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // for ec in eg:Iter() do
    let eg = a.eg.clone().nil();
    for ec in group::iter(d, &eg) {
        // if ec:IsSetCard(SET_ULTIMATE_CRYSTAL) and ec:IsFaceup() then
        if card::is_set_card(d, ec, &[SET_ULTIMATE_CRYSTAL as u16]) && card::is_faceup(d, ec) {
            // Duel.RegisterFlagEffect(ec:GetSummonPlayer(),id,0,0,0)
            let p = card::get_summon_player(d, ec);
            duel::register_flag_effect(d, p, ID, 0, 0, 0, 0);
        }
    }
}
/// `s.cbfilter(c)`
fn cbfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(s.cbfilter,e:GetHandlerPlayer(),LOCATION_REMOVED,0,nil):GetClassCount(Card.GetCode)>=7
    let p = effect::get_handler_player(d, a.e);
    let g = duel::get_matching_group(d, Some(&cbfilter), p, LOCATION_REMOVED, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 7
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return s.cbfilter(c) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    cbfilter(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if chk==0 then return #g>0 and Duel.GetMZoneCount(tp,g+c)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let mut gc = g.clone();
        group::add_card(&mut gc, &CardSet::from_card(c));
        return !g.is_empty()
            && duel::get_m_zone_count(d, tp, (&gc).into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g<=0 or Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)<=0 then return end
    if g.is_empty() || duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) <= 0 {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,ft,nil,e,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_REMOVED, 0, 1, ft as u16, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
