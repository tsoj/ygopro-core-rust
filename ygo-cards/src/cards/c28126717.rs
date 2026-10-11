//! Floowandereeze and the Magnificent Map
//! ふわんだりぃずと謎の地図
use super::prelude::*;

pub const ID: u32 = 28126717;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOOWANDEREEZE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reveal 1 Level 1 "Floowandereeze" monster in your hand and banish 1 "Floowandereeze" card with a different name from your Deck then immediately after this effect resolves, Normal Summon the revealed monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Immediately after this effect resolves, Normal Summon 1 "Floowandereeze" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e3, CATEGORY_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.nscon)
    effect::set_condition(d, e3, Some(condition(nscon)));
    // e3:SetTarget(s.nstg)
    effect::set_target(d, e3, Some(cost_target(nstg)));
    // e3:SetOperation(s.nsop)
    effect::set_operation(d, e3, Some(operation(nsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FLOOWANDEREEZE) and c:IsLevel(1) and c:IsSummonable(true,nil) and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    card::is_set_card(d, c, &[SET_FLOOWANDEREEZE as u16])
        && card::is_level(d, c, &[1])
        && card::is_summonable(d, c, true, None, 0, None)
        && !card::is_public(d, c)
        && {
            let code = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.filter2(c,code)`
fn filter2(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsSetCard(SET_FLOOWANDEREEZE) and c:IsAbleToRemove() and not c:IsCode(code)
    card::is_set_card(d, c, &[SET_FLOOWANDEREEZE as u16]) && card::is_able_to_remove(d, c, None, None, None) && !card::is_code(d, c, &[code])
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.ConfirmCards(1-tp,g1)
        duel::confirm_cards(d, 1 - tp, g1.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,g1:GetFirst():GetCode())
        let first = group::get_first(d, &g1).nil();
        let code = card::get_code(d, first)[0];
        let g2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g2>0 and Duel.Remove(g2,POS_FACEUP,REASON_EFFECT)>0 then
        if !g2.is_empty() && duel::remove(d, g2, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local sg=g1:GetFirst(tp,1,1,nil)
            let sg = group::get_first(d, &g1).nil();
            // Duel.Summon(tp,sg,true,nil)
            duel::summon(d, tp, sg, true, None, 0, None);
        }
    }
}
/// `s.nscon(e,tp,eg,ep,ev,re,r,rp)`
fn nscon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and ep==1-tp
    a.eg.is_some() && a.ep == 1 - a.tp
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FLOOWANDEREEZE) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_FLOOWANDEREEZE as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local sg=g:GetFirst(tp,1,1,nil)
        let sg = group::get_first(d, &g).nil();
        // Duel.Summon(tp,sg,true,nil)
        duel::summon(d, tp, sg, true, None, 0, None);
    }
}
