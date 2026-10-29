//! Zoodiac Hammerkong
use super::prelude::*;

pub const ID: u32 = 14970113;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZOODIAC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,3,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(4),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk
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
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(effect_card_fn(efftg)));
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // remove material
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.rmtg)
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    // e4:SetOperation(s.rmop)
    effect::set_operation(d, e4, Some(operation(rmop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ZOODIAC,lc,SUMMON_TYPE_XYZ,tp) and not c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,id)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_ZOODIAC as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // return true
    true
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:GetAttack()>=0
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::get_attack(d, c) >= 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.atkfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| atkfilter(d, c), NO_EX);
    // return g:GetSum(Card.GetAttack)
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64)
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:GetDefense()>=0
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::get_defense(d, c) >= 0
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.deffilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| deffilter(d, c), NO_EX);
    // return g:GetSum(Card.GetDefense)
    group::get_sum(d, &g, &|d, c| card::get_defense(d, c) as i64)
}
/// `s.efftg(e,c)`
fn efftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && c != h
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) > 0
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
    card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
}
