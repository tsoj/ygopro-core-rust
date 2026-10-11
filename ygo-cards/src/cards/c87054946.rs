//! Nephthys, the Sacred Flame
use super::prelude::*;

pub const ID: u32 = 87054946;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEPHTHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon procedure
    // Link.AddProcedure(c,nil,2,99,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(99), Some(std::sync::Arc::new(|d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8| lcheck(d, g, lc, st, tp))), None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // effect gain
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.matcheck)
    effect::set_value_fn(d, e1, effect_card_value(matcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot select battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(condition(atcon)));
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_RITUAL,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_RITUAL, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local ct=c:GetMaterial():FilterCount(Card.IsType,nil,TYPE_RITUAL)
    let mat = card::get_material(d, c);
    let ct = group::filter_count(d, &mat, &|d, x| card::is_type(d, x, TYPE_RITUAL), NO_EX);
    // if ct>=1 then
    if ct >= 1 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
    }
    // if ct>=2 then
    if ct >= 2 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetValue(1200)
        effect::set_value(d, e2, 1200);
        // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    }
    // if ct==3 then
    if ct == 3 {
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e4:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
        // e4:SetRange(LOCATION_MZONE)
        effect::set_range(d, e4, LOCATION_MZONE);
        // e4:SetValue(1200)
        effect::set_value(d, e4, 1200);
        // e4:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e4, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e4)
        card::register_effect(d, c, e4, false);
        // local e5=Effect.CreateEffect(c)
        let e5 = effect::create_effect(d, c);
        // e5:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e5:SetRange(LOCATION_MZONE)
        effect::set_range(d, e5, LOCATION_MZONE);
        // e5:SetValue(1)
        effect::set_value(d, e5, 1);
        // e5:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e5, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e5)
        card::register_effect(d, c, e5, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    }
    0
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSequence()>4
    let h = effect::get_handler(d, a.e).nil();
    card::get_sequence(d, h) > 4
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_NEPHTHYS) and c:GetSequence()<5
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && card::get_sequence(d, c) < 5
}
