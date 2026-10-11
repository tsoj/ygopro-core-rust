//! Double Exposure
//! Scripted by Eerie Code, based on the anime version
use super::prelude::*;

pub const ID: u32 = 81881839;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_CIPHER}
    listed_series: &[SET_CIPHER],
    ..ScriptInfo::DEFAULT
};

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
    // double level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // name change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE_START);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.nmtg)
    effect::set_target(d, e3, Some(cost_target(nmtg)));
    // e3:SetOperation(s.nmop)
    effect::set_operation(d, e3, Some(operation(nmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lvfilter(c,e)`
fn lvfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(6) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_level_below(d, c, 6) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.lvcheck(sg,e,tp,mg)`
fn lvcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==1
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1, false)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| lvfilter(d, c, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.lvcheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&lvcheck), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.lvcheck,1,tp,HINTMSG_FACEUP)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&lvcheck), 1, tp, Some(HINTMSG_FACEUP as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let tg0 = duel::get_target_cards(d, e).nil();
    let tg = group::filter(d, &tg0, &|d, x| card::is_faceup(d, x), NO_EX);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // --Double their Levels
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
        // e1:SetValue(tc:GetLevel()*2)
        let lv = card::get_level(d, tc) as i64;
        effect::set_value(d, e1, lv * 2);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.nmfilter2(c,cd)`
fn nmfilter2(d: &mut Duel, c: CardId, cd: &[u32]) -> bool {
    // return c:IsFaceup() and not c:IsCode(cd)
    card::is_faceup(d, c) && !card::is_code(d, c, cd)
}
/// `s.nmfilter(c)`
fn nmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CIPHER)
    //     and Duel.IsExistingMatchingCard(s.nmfilter2,0,LOCATION_MZONE,LOCATION_MZONE,1,c,c:GetCode())
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CIPHER as u16])) {
        return false;
    }
    let codes = card::get_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| nmfilter2(d, x, &codes)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.nmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn nmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.nmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && nmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.nmfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&nmfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.nmfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&nmfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.nmop(e,tp,eg,ep,ev,re,r,rp)`
fn nmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.nmfilter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,tc,tc:GetCode())
    let tc_codes = card::get_code(d, tc);
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| nmfilter2(d, x, &tc_codes)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, tc.into()).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc:GetCode())
        let code = card::get_code(d, tc)[0] as i64;
        effect::set_value(d, e1, code);
        // sg:GetFirst():RegisterEffect(e1)
        let first = group::get_first(d, &sg).nil();
        card::register_effect(d, first, e1, false);
    }
}
