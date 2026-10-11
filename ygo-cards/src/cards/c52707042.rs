//! Libromancer Mystigirl
use super::prelude::*;

pub const ID: u32 = 52707042;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LIBROMANCER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Check materials on Ritual Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.matcheck)
    effect::set_value_fn(d, e0, lua_fn(matcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Prevent effect target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.immcon)
    effect::set_condition(d, e1, Some(condition(immcon)));
    // e1:SetTarget(function(e,c) return c:IsRitualMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_ritual_monster(d, c))));
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change ATK to 0 and negate effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matcheck(e,c)
fn matcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    // if c:GetMaterial():IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, x| card::is_location(d, x, LOCATION_MZONE), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
    }
}
// function s.immcon(e)
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRitualSummoned() and c:GetFlagEffect(id)>0
    card::is_ritual_summoned(d, c) && card::get_flag_effect(d, c, ID) > 0
}
// function s.immval(e,re,rp)
fn immval(d: &mut Duel, a: Args) -> bool {
    let (e, re, rp) = (a.e(0), a.e(1), a.p(2));
    // return aux.tgoval(e,re,rp) and re:IsMonsterEffect()
    aux::tgoval().call(d, &[Value::Effect(e), Value::Effect(re), Value::Int(rp as i64)]).truthy() && effect::is_monster_effect(d, re)
}
// function s.disfilter(c)
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (not c:IsAttack(0) or c:IsNegatableMonster())
    card::is_faceup(d, c) && (!card::is_attack(d, c, &[0]) || card::is_negatable_monster(d, c))
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.disfilter(chkc) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && disfilter(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Change its ATK to 0
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Negate its effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetValue(RESET_TURN_SET)
        effect::set_value(d, e3, RESET_TURN_SET as i64);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
