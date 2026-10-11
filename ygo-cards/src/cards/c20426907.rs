//! Re-qliate
//! 機殻の再星
use super::prelude::*;

pub const ID: u32 = 20426907;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_QLI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e4, CATEGORY_DISABLE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTarget(s.distg2)
    effect::set_target(d, e4, Some(cost_target(distg2)));
    // e4:SetOperation(s.disop2)
    effect::set_operation(d, e4, Some(operation(disop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // send itself to the Grave
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e5, EFFECT_SELF_TOGRAVE);
    // e5:SetCondition(s.sdcon)
    effect::set_condition(d, e5, Some(condition(sdcon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local eg = a.eg (the event group)
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        // if e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED) then
        let h = effect::get_handler(d, a.e).nil();
        let tc = group::get_first(d, &eg).nil();
        if card::is_status(d, h, STATUS_EFFECT_ENABLED) {
            // return eg:GetFirst():IsLevelBelow(4)
            return card::is_level_below(d, tc, 4);
        } else {
            // return not eg:GetFirst():IsType(TYPE_NORMAL) and eg:GetFirst():IsLevelBelow(4)
            return !card::is_type(d, tc, TYPE_NORMAL) && card::is_level_below(d, tc, 4);
        }
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(eg), 1, 0, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
// function s.ngtfilter(c,check)
fn ngtfilter(d: &mut Duel, c: CardId, check: bool) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(5) and (not c:IsType(TYPE_NORMAL) or check)
    card::is_faceup(d, c) && card::is_level_above(d, c, 5) && (!card::is_type(d, c, TYPE_NORMAL) || check)
}
// function s.distg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // local check=e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED)
    let h = effect::get_handler(d, a.e).nil();
    let check = card::is_status(d, h, STATUS_EFFECT_ENABLED);
    // if chk==0 then return eg:IsExists(s.ngtfilter,1,nil,check) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| ngtfilter(d, c, check), 1, NO_EX);
    }
    // local gp=eg:Filter(s.ngtfilter,nil,check)
    let gp = group::filter(d, &eg, &|d, c| ngtfilter(d, c, check), NO_EX);
    // Duel.SetTargetCard(gp)
    duel::set_target_card(d, gp.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,gp,#gp,0,0)
    let n = gp.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(gp), n, 0, 0);
    true
}
// function s.disop2(e,tp,eg,ep,ev,re,r,rp)
fn disop2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local gp=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let tg = duel::get_target_cards(d, a.e).nil();
    let gp = group::filter(d, &tg, &|d, c| card::is_faceup(d, c), NO_EX);
    // local tc=gp:GetFirst()
    let _tc = group::get_first(d, &gp);
    // for tc in aux.Next(gp) do
    for tc in group::iter(d, &gp) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_REDIRECT, 1);
        // e3:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e3, LOCATION_REMOVED as i64);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
// function s.sdfilter(c)
fn sdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_QLI) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_QLI as u16]) && !card::is_code(d, c, &[ID])
}
// function s.sdcon(e)
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.sdfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&sdfilter), p, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
