//! Vain Betrayer
use super::prelude::*;

pub const ID: u32 = 94933468;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(aux.PersistentTgCon)
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    // e2:SetOperation(aux.PersistentTgOp(false))
    effect::set_operation(d, e2, Some(aux::persistent_tg_op(false)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // eff
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3, EFFECT_DISABLE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetCondition(s.descon)
    effect::set_condition(d, e5, Some(condition(descon)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // discard deck
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e6, CATEGORY_DECKDES);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.deckcon)
    effect::set_condition(d, e6, Some(condition(deckcon)));
    // e6:SetTarget(s.decktg)
    effect::set_target(d, e6, Some(cost_target(decktg)));
    // e6:SetOperation(s.deckop)
    effect::set_operation(d, e6, Some(operation(deckop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}

/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if chk==0 then return at:IsControler(1-tp) and at:IsType(TYPE_XYZ) and at:IsOnField() and at:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return card::is_controler(d, at, 1 - tp)
            && card::is_type(d, at, TYPE_XYZ)
            && card::is_on_field(d, at)
            && card::is_can_be_effect_target(d, at, Some(a.e));
    }
    // Duel.SetTargetCard(at)
    duel::set_target_card(d, at);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,at,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(at)), 1, 0, 0);
    true
}

/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED as u32) {
        return false;
    }
    // local tc=c:GetFirstCardTarget()
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, c) {
        Some(tc) => a.eg.clone().nil().contains(tc),
        None => false,
    }
}

/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}

/// `s.deckcon(e,tp,eg,ep,ev,re,r,rp)`
fn deckcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFirstCardTarget()~=nil
    let c = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, 1 - a.tp) && card::get_first_card_target(d, c).is_some()
}

/// `s.decktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,1-tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 3);
    true
}

/// `s.deckop(e,tp,eg,ep,ev,re,r,rp)`
fn deckop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // Duel.DiscardDeck(1-tp,3,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 3, REASON_EFFECT);
}
