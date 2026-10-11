//! Amazoness Pet Liger
//! アマゾネスペット虎獅子
use super::prelude::*;

pub const ID: u32 = 68507541;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], material_setcode: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,10979723,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_AMAZONESS))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(10979723),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_AMAZONESS as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // --atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.atkcon1)
    effect::set_condition(d, e1, Some(lua_fn(atkcon1)));
    // e1:SetOperation(s.atkop1)
    effect::set_operation(d, e1, Some(operation(atkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --atk down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(s.atkcon2)
    effect::set_condition(d, e2, Some(condition(atkcon2)));
    // e2:SetTarget(s.atktg2)
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    // e2:SetOperation(s.atkop2)
    effect::set_operation(d, e2, Some(operation(atkop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --atk target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetValue(s.atktg)
    effect::set_value_fn(d, e3, effect_card_fn(atktg));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon1(e)`
fn atkcon1(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler()==Duel.GetAttacker()
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::get_attacker(d) == Some(h)
}
/// `s.atkop1(e,tp,eg,ep,ev,re,r,rp)`
fn atkop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.atkcon2(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let bt = card::get_battle_target(d, att);
    // return a:IsControler(tp) and a:IsSetCard(SET_AMAZONESS) and d and d:IsControler(1-tp)
    if !card::is_controler(d, att, tp) {
        return false;
    }
    if !card::is_set_card(d, att, &[SET_AMAZONESS as u16]) {
        return false;
    }
    match bt {
        Some(bt) => card::is_controler(d, bt, 1 - tp),
        None => false,
    }
}
/// `s.atktg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop2(e,tp,eg,ep,ev,re,r,rp)`
fn atkop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(-800)
            effect::set_value(d, e1, -800);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZONESS) and c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && c != h
}
