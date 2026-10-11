//! Dracotail Shaulas
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 42125140;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Dragontail" monster + 1 monster in the hand
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DRACOTAIL),aux.FilterBoolFunctionEx(Card.IsLocation,LOCATION_HAND))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DRACOTAIL as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_location(d, c, LOCATION_HAND))),
        ],
    );
    // Shuffle 2 "Dracotail" cards from your GY, and 1 face-up card on the field of the same type (Monster, Spell, or Trap) as 1 of them into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from the GY, but banish it when it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of monsters being sent to the GY at the same time
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(function() return not Duel.IsDamageStep() end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetOperation(function(e) e:GetHandler():ResetFlagEffect(id+1) end)
    effect::set_operation(d, e4, Some(operation(e4_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e5, EVENT_CHAIN_SOLVED);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EVENT_BREAK_EFFECT)
    effect::set_code(d, e6, EVENT_BREAK_EFFECT);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.tdfilter(c,e)
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return (c:IsSetCard(SET_DRACOTAIL) or c:IsOnField()) and c:IsFaceup() and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    (card::is_set_card(d, c, &[SET_DRACOTAIL as u16]) || card::is_on_field(d, c))
        && card::is_faceup(d, c)
        && card::is_able_to_deck(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local gy_sg,field_sg=sg:Split(Card.IsLocation,nil,LOCATION_GRAVE)
    let (gy_sg, field_sg) = group::split(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // return #gy_sg==2 and #field_sg==1 and field_sg:GetFirst():IsType(gy_sg:GetBitwiseOr(Card.GetMainCardType))
    let res = if gy_sg.len() == 2 && field_sg.len() == 1 {
        let fc = group::get_first(d, &field_sg).nil();
        let mask = group::get_bitwise_or(d, &gy_sg, &|d, c| card::get_main_card_type(d, c) as i64);
        card::is_type(d, fc, mask as u32)
    } else {
        false
    };
    (res, false)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE | LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 3
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TODECK)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 3, tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==3 then
    if tg.len() == 3 {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=eg:FilterCount(Card.IsMonster,nil)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, cc| card::is_monster(d, cc), NO_EX);
    // if ct==0 or eg:IsContains(c) then return end
    if ct == 0 || eg.contains(c) {
        return;
    }
    // if ct>=2 or c:HasFlagEffect(id+1) then
    if ct >= 2 || card::has_flag_effect(d, c, ID + 1, None) {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,ep,ev)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r as u32, a.rp, a.ep, a.ev as u32);
    }
    // local eff=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_EFFECT)
    let eff = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_EFFECT])[0].clone();
    // if eff then
    if eff.truthy() {
        // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
        card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    }
}
// function(e) e:GetHandler():ResetFlagEffect(id+1) end
fn e4_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, ID + 1);
}
// function() return not Duel.IsDamageStep() end
fn e3_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_damage_step(d, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
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
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // --Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
