//! Vendread Nightmare
use super::prelude::*;

pub const ID: u32 = 33971095;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

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
    // Increase the level of 1 face-up monster you control by the number of monsters Tributed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.lvcost)
    effect::set_cost(d, e2, Some(cost_target(lvcost)));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You attacking "Vendread" Ritual Monster gains 1000 ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.lvcfilter(c)
fn lvcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_VENDREAD)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_VENDREAD as u16])
}
// function s.lvfilter(c,e)
fn lvfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(1) and (not e or c:IsCanBeEffectTarget(e))
    card::is_faceup(d, c) && card::is_level_above(d, c, 1) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.lvcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil,e)
    let tg = duel::get_matching_group(d, Some(&|d, c| lvfilter(d, c, Some(e))), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.lvcfilter,1,true,aux.ReleaseCheckTarget,nil,tg) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(
            d,
            tp,
            Some(&lvcfilter),
            1,
            1,
            true,
            Some(&|d: &mut Duel, sel: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sel, tp, exg, &tg)),
            NO_EX,
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.lvcfilter,1,99,true,aux.ReleaseCheckTarget,nil,tg)
    let g = aux::select_release_group_cost(
        d,
        tp,
        Some(&lvcfilter),
        1,
        99,
        true,
        Some(&|d: &mut Duel, sel: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sel, tp, exg, &tg)),
        NO_EX,
    );
    // e:SetLabel(#g)
    effect::set_label(d, e, &[g.len() as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.lvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && lvfilter(d, chkc, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // local g=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| lvfilter(d, c, None)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,g,1,tp,e:GetLabel())
    let lbl = effect::get_label(d, e) as i32;
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(g), 1, tp, lbl);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(e:GetLabel())
        let lbl = effect::get_label(d, a.e);
        effect::set_value(d, e1, lbl);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // return a:IsControler(tp) and (a:IsRitualMonster() and a:IsSetCard(SET_VENDREAD)) and a:GetBattleTarget():IsControler(1-tp)
    card::is_controler(d, at, tp)
        && (card::is_ritual_monster(d, at) && card::is_set_card(d, at, &[SET_VENDREAD as u16]))
        && card::is_controler(d, card::get_battle_target(d, at).nil(), 1 - tp)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToBattle() and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_battle(d, tc.nil()) && card::is_faceup(d, tc.nil()) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
