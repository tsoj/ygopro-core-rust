//! Advanced Crystal Beast Amethyst Cat
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 19963185;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ADVANCED_DARK], listed_series: &[SET_ADVANCED_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // Send itself to the GY if "Adanced Dark" is not face-up in the Field Spell Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e1, EFFECT_SELF_TOGRAVE);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place itself in the S/T instead of sending it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCondition(s.replacecon)
    effect::set_condition(d, e2, Some(condition(replacecon)));
    // e2:SetOperation(s.replaceop)
    effect::set_operation(d, e2, Some(operation(replaceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // "Advanced Crystal Beast" monster can attack directly
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ADVANCED_CRYSTAL_BEAST))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ADVANCED_CRYSTAL_BEAST as u16]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Battle damage is halved if attacking directly
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e4, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ADVANCED_CRYSTAL_BEAST))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ADVANCED_CRYSTAL_BEAST as u16]))));
    // e4:SetCondition(s.rdmgcond)
    effect::set_condition(d, e4, Some(condition(rdmgcond)));
    // e4:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e4, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return not Duel.IsEnvironment(CARD_ADVANCED_DARK)
    !duel::is_environment(d, CARD_ADVANCED_DARK, None, None)
}
/// `s.replacecon(e)`
fn replacecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.replaceop(e,tp,eg,ep,ev,re,r,rp)`
fn replaceop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, CardSet::from_card(c), EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.rdmgcond(e)`
fn rdmgcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // local c=Duel.GetAttacker()
    // return Duel.GetAttackTarget()==nil and c:GetEffectCount(EFFECT_DIRECT_ATTACK)<2
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_attack_target(d).is_none()
        && card::get_effect_count(d, duel::get_attacker(d).nil(), EFFECT_DIRECT_ATTACK) < 2
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
