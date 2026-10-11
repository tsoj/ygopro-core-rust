//! Gunkan Suship Uni-class Super-Dreadnought
use super::prelude::*;

pub const ID: u32 = 94798725;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GUNKAN],
    listed_names: &[CARD_SUSHIP_SHARI, CARD_SUSHIP_UNI],
    ..ScriptInfo::DEFAULT
};
/// `local CARD_SUSHIP_UNI=42377643`
const CARD_SUSHIP_UNI: u32 = 42377643;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon.
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate card effects.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(TIMING_SPSUMMON,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, TIMING_SPSUMMON, Some(TIMING_BATTLE_START));
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains effects based on material.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return (Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()) or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase())
    (duel::is_turn_player(d, tp) && duel::is_main_phase(d, None)) || (duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsNegatable() and chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD) end
    if let Some(chkc) = a.chkc() {
        return card::is_negatable(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_ONFIELD);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, x| card::is_negatable(d, x)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_negatable(d, x)), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(e:GetHandler(),RESET_EVENT|RESETS_STANDARD,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_EVENT | RESETS_STANDARD), true, None);
    }
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) > 0
}
/// `s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // if chk==0 then return ((effs&1)>0 and Duel.IsPlayerCanDraw(tp,1)) or ((effs&2)>0) end
    if a.chk == Some(0) {
        return ((effs & 1) > 0 && duel::is_player_can_draw(d, tp, 1)) || ((effs & 2) > 0);
    }
    // if (effs&1)>0 then
    if (effs & 1) > 0 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // "Gunkan Suship Shari": Draw 1 card.
    // if (effs&1)>0 then
    if (effs & 1) > 0 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
    // "Gunkan Suship Uni: This card can attack directly.
    // if (effs&2)>0 then
    if (effs & 2) > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local effs=0
    let mut effs = 0;
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_SHARI) then effs=effs|1 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_SHARI]), 1, NO_EX) {
        effs |= 1;
    }
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_UNI) then effs=effs|2 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_UNI]), 1, NO_EX) {
        effs |= 2;
    }
    // e:GetLabelObject():SetLabel(effs)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[effs]);
    0
}
