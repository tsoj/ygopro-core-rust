//! Performapal Swincobra
use super::prelude::*;

pub const ID: u32 = 93892436;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --deckdes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.ddescon)
    effect::set_condition(d, e1, Some(condition(ddescon)));
    // e1:SetTarget(s.ddestg)
    effect::set_target(d, e1, Some(cost_target(ddestg)));
    // e1:SetOperation(s.ddesop)
    effect::set_operation(d, e1, Some(operation(ddesop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --direct attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --change position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.poscon)
    effect::set_condition(d, e3, Some(condition(poscon)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.ddescon(e,tp,eg,ep,ev,re,r,rp)
fn ddescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and eg:GetFirst():IsControler(tp)
    if a.ep == a.tp {
        return false;
    }
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    card::is_controler(d, tc, a.tp)
}
// s.ddestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ddestg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 1);
    true
}
// s.ddesop(e,tp,eg,ep,ev,re,r,rp)
fn ddesop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.DiscardDeck(1-tp,1,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 1, REASON_EFFECT);
}
// s.poscon(e,tp,eg,ep,ev,re,r,rp)
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetAttackedCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, c) > 0
}
// s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
