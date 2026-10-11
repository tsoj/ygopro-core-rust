//! Ghostrick Museum
//! ゴーストリック・ミュージアム
use super::prelude::*;

pub const ID: u32 = 7617062;

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
    // cannot attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.ftarget)
    effect::set_target(d, e2, Some(effect_card_fn(ftarget)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atklimit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetValue(s.bttg)
    effect::set_value_fn(d, e3, effect_card_fn(bttg));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // direct attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_DIRECT_ATTACK);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(s.dirtg)
    effect::set_target(d, e4, Some(effect_card_fn(dirtg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // pos
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e5, EVENT_BATTLE_DAMAGE);
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e6, CATEGORY_POSITION + CATEGORY_SET);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e6, EVENT_DAMAGE_STEP_END);
    // e6:SetCondition(s.poscon)
    effect::set_condition(d, e6, Some(condition(poscon)));
    // e6:SetOperation(s.posop)
    effect::set_operation(d, e6, Some(operation(posop)));
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.bttg(e,c)`: `return c:IsFacedown()`
fn bttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_facedown(d, c)
}
/// `s.dirtg(e,c)`
fn dirtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,c:GetControler(),0,LOCATION_MZONE,1,nil)
    let p = card::get_controler(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.ftarget(e,c)`: `return not c:IsSetCard(SET_GHOSTRICK)`
fn ftarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    // eg:GetFirst():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::register_flag_effect(d, tc, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`: `return e:GetHandler():GetFlagEffect(id)>0`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.filter(c)`: `return c:GetFlagEffect(id+1)>0`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID + 1) > 0
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
