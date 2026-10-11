//! ターボ・ウォリアー (Turbo Warrior)
use super::prelude::*;

pub const ID: u32 = 46195773;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[67270095],
    listed_names: &[67270095],
    material_setcode: &[SET_SYNCHRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,s.tfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(tfilter)), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk down
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,re,rp) local lvl=re:GetCardLevel() return lvl>0 and lvl<=6 end)
    effect::set_value_fn(d, e2, lua_fn(cannot_target_val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tfilter(c,scard,sumtype,tp)`
fn tfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsSummonCode(scard,sumtype,tp,67270095) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, sc, st, tp, &[67270095]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let tgt = duel::get_attack_target(d);
    // if chk==0 then return d and d:IsFaceup() and d:GetLevel()>=6 and d:IsType(TYPE_SYNCHRO) end
    if a.chk == Some(0) {
        return match tgt {
            None => false,
            Some(t) => card::is_faceup(d, t) && card::get_level(d, t) >= 6 && card::is_type(d, t, TYPE_SYNCHRO),
        };
    }
    // d:CreateEffectRelation(e)
    let t = tgt.nil();
    card::create_effect_relation(d, t, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(t)), 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // if d:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, t, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(d:GetAttack()/2)
        let atk = card::get_attack(d, t);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // d:RegisterEffect(e1)
        card::register_effect(d, t, e1, false);
    }
}
/// `function(e,re,rp) local lvl=re:GetCardLevel() return lvl>0 and lvl<=6 end` (e2)
fn cannot_target_val(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // local lvl=re:GetCardLevel()
    let lvl = match chain::effect_get_card(d, re, chain::Prop::Level) {
        Value::Nil => crate::lua_error("attempt to compare nil with number"),
        v => v.int(),
    };
    // return lvl>0 and lvl<=6
    lvl > 0 && lvl <= 6
}
