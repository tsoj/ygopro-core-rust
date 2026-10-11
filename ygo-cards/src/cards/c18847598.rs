//! Advanced Crystal Beast Amber Mammoth
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 18847598;

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
    // e1:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e1, EFFECT_SELF_TOGRAVE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return not Duel.IsEnvironment(CARD_ADVANCED_DARK) end)
    effect::set_condition(d, e1, Some(condition(|d, _a| !duel::is_environment(d, CARD_ADVANCED_DARK, None, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place itself in the S/T instead of sending it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e2:SetCondition(s.replacecon)
    effect::set_condition(d, e2, Some(condition(replacecon)));
    // e2:SetOperation(s.replaceop)
    effect::set_operation(d, e2, Some(operation(replaceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate attack on a "Advanced Crystal Beast" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negatkcon)
    effect::set_condition(d, e3, Some(condition(negatkcon)));
    // e3:SetOperation(function() Duel.NegateAttack() end)
    effect::set_operation(d, e3, Some(operation(|d, _a| {
        duel::negate_attack(d);
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the activation of an effect that targets "Advanced Dark" or "Advanced Crystal Beast" cards
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e4, CATEGORY_NEGATE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.negcon)
    effect::set_condition(d, e4, Some(condition(negcon)));
    // e4:SetTarget(s.negtg)
    effect::set_target(d, e4, Some(cost_target(negtg)));
    // e4:SetOperation(function(e,tp,eg,ep,ev) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e4, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
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
    // Treated as a Continuous Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, CardSet::from_card(c), EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.negatkcon(e,tp,eg,ep,ev,re,r,rp)`
fn negatkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // return d:IsFaceup() and d:IsSetCard(SET_ADVANCED_CRYSTAL_BEAST) and d:IsControler(tp)
    card::is_faceup(d, t) && card::is_set_card(d, t, &[SET_ADVANCED_CRYSTAL_BEAST as u16]) && card::is_controler(d, t, a.tp)
}
/// `s.advfilter(c,tp)`
fn advfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsFaceup() and (c:IsSetCard(SET_ADVANCED_CRYSTAL_BEAST) or c:IsCode(CARD_ADVANCED_DARK))
    card::is_controler(d, c, tp) && card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_ADVANCED_CRYSTAL_BEAST as u16]) || card::is_code(d, c, &[CARD_ADVANCED_DARK]))
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and rp==1-tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)) then return false end
    let re = a.re.nil();
    let h = effect::get_handler(d, a.e).nil();
    let ok = effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)
        && a.rp == 1 - a.tp
        && !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8);
    if !ok {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return g and g:IsExists(s.advfilter,1,nil,tp)
    match g {
        Some(g) => {
            let tp = a.tp;
            group::is_exists(d, &g, &|d, c| advfilter(d, c, tp), 1, NO_EX)
        }
        None => false,
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
