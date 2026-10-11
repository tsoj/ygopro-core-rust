//! Defender of Nephthys
use super::prelude::*;

pub const ID: u32 = 51782995;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_NEPHTHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 card in your hand, and if you do, Special Summon 1 Level 4 or lower "Nephthys" monster from your hand, except "Defender of Nephthys"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register when this card is destroyed
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2a, EVENT_DESTROYED);
    // e2a:SetOperation(function(e,tp) ... end)
    effect::set_operation(d, e2a, Some(operation(reg_destroyed_op)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Destroy 1 "Nephthys" monster in your Deck, except "Defender of Nephthys"
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2b, aux::stringid(ID, 1));
    // e2b:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2b, CATEGORY_DESTROY);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e2b, EVENT_PHASE + PHASE_STANDBY);
    // e2b:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2b, LOCATION_GRAVE);
    // e2b:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2b, 1, ID, 1, 0);
    // e2b:SetCondition(function(e,tp) ... end)
    effect::set_condition(d, e2b, Some(condition(reg_cond)));
    // e2b:SetTarget(s.destg)
    effect::set_target(d, e2b, Some(cost_target(destg)));
    // e2b:SetOperation(s.desop)
    effect::set_operation(d, e2b, Some(operation(desop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
// e2a:SetOperation(function(e,tp) local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1 e:GetHandler():RegisterFlagEffect(...) end)
fn reg_destroyed_op(d: &mut Duel, a: &EffArgs) {
    // local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1
    let ct: u16 = if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_STANDBY) { 2 } else { 1 };
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,ct,ct)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 0, ct, ct as i64, 0);
}
// e2b:SetCondition(function(e,tp) local c=e:GetHandler() return Duel.IsTurnPlayer(tp) and c:IsReason(REASON_EFFECT) and c:HasFlagEffect(id) and (c:GetFlagEffectLabel(id)==1 or c:GetTurnID()~=Duel.GetTurnCount()) end)
fn reg_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_flag_effect(d, c, ID) > 0
        && (card::get_flag_effect_label(d, c, ID).first().copied() == Some(1)
            || card::get_turn_id(d, c) as i32 != duel::get_turn_count(d, None))
}
// function s.handdesfilter(c,e,tp,res_chk)
fn handdesfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, res_chk: bool) -> bool {
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,c,e,tp) or res_chk
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into()) || res_chk
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_NEPHTHYS) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_NEPHTHYS as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.handdesfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| handdesfilter(d, c, a.e, a.tp, false)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local res_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or not Duel.IsExistingMatchingCard(s.handdesfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let res_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_existing_matching_card(d, Some(&|d, c| handdesfilter(d, c, a.e, a.tp, false)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.handdesfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,res_chk)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| handdesfilter(d, c, a.e, a.tp, res_chk)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// function s.deckdesfilter(c)
fn deckdesfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEPHTHYS) and c:IsMonster() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID])
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckdesfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckdesfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckdesfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&deckdesfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
