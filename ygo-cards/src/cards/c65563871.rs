//! Sunvine Healer
use super::prelude::*;

pub const ID: u32 = 65563871;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUNAVALON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetTarget(s.rectg)
    effect::set_target(d, e2, Some(cost_target(rectg)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // recover
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetCondition(s.reccon2)
    effect::set_condition(d, e3, Some(condition(reccon2)));
    // e3:SetTarget(s.rectg2)
    effect::set_target(d, e3, Some(cost_target(rectg2)));
    // e3:SetOperation(s.recop2)
    effect::set_operation(d, e3, Some(operation(recop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matfilter(c,lc,st,tp)
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT,lc,st,tp) and c:IsType(TYPE_NORMAL,lc,st,tp)
    card::is_race_ctx(d, c, RACE_PLANT, lc, st, Some(tp)) && card::is_type_ctx(d, c, TYPE_NORMAL, lc, st, Some(tp))
}
// function s.cfilter(c,tp,rp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsSetCard(SET_SUNAVALON) and c:IsType(TYPE_LINK) and c:GetPreviousControler()==tp and c:IsReason(REASON_EFFECT)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_set_card(d, c, &[SET_SUNAVALON as u16])
        && card::is_type(d, c, TYPE_LINK)
        && card::get_previous_controler(d, c) == tp
        && card::is_reason(d, c, REASON_EFFECT)
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp,rp)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
// function s.recfilter(c)
fn recfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SUNAVALON) and c:IsType(TYPE_LINK)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUNAVALON as u16]) && card::is_type(d, c, TYPE_LINK)
}
// function s.rectg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.recfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && recfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.recfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&recfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.recfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&recfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,g:GetFirst():GetLink()*300)
    let tc = group::get_first(d, &g).nil();
    let v = card::get_link(d, tc) * 300;
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, v as i32);
    true
}
// function s.recop(e,tp,eg,ep,ev,re,r,rp)
fn recop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:GetLink()>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::get_link(d, tc) > 0 {
            // Duel.Recover(tp,tc:GetLink()*300,REASON_EFFECT)
            let v = card::get_link(d, tc) as i64 * 300;
            duel::recover(d, a.tp, v, REASON_EFFECT, false, None);
        }
    }
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsType(TYPE_LINK) and c:IsRace(RACE_PLANT)
    card::is_controler(d, c, tp) && card::is_type(d, c, TYPE_LINK) && card::is_race(d, c, RACE_PLANT)
}
// function s.reccon2(e,tp,eg,ep,ev,re,r,rp)
fn reccon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and eg:IsExists(s.filter,1,nil,tp)
    let tp = a.tp;
    a.ep != tp && group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, tp), 1, NO_EX)
}
// function s.rectg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn rectg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(600)
    duel::set_target_param(d, 600);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,600)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 600);
    true
}
// function s.recop2(e,tp,eg,ep,ev,re,r,rp)
fn recop2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
