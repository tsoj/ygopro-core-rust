//! Crystal Beast Amber Mammoth
use super::prelude::*;

pub const ID: u32 = 69937550;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCondition(s.repcon)
    effect::set_condition(d, e1, Some(condition(repcon)));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.cbcon)
    effect::set_condition(d, e2, Some(condition(cbcon)));
    // e2:SetTarget(s.cbtg)
    effect::set_target(d, e2, Some(cost_target(cbtg)));
    // e2:SetOperation(s.cbop)
    effect::set_operation(d, e2, Some(operation(cbop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.repcon(e)
fn repcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
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
    duel::raise_event(d, c, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
}
// function s.cbcon(e,tp,eg,ep,ev,re,r,rp)
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bt=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bt = group::get_first(d, &eg);
    // return r~=REASON_REPLACE and c~=bt and bt:IsFaceup() and bt:IsSetCard(SET_CRYSTAL_BEAST) and bt:GetControler()==c:GetControler()
    if a.r == REASON_REPLACE || Some(c) == bt {
        return false;
    }
    let bt = bt.nil();
    card::is_faceup(d, bt) && card::is_set_card(d, bt, &[SET_CRYSTAL_BEAST as u16]) && card::get_controler(d, bt) == card::get_controler(d, c)
}
// function s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():GetAttackableTarget():IsContains(e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let att = duel::get_attacker(d).nil();
        let (ag, _) = card::get_attackable_target(d, att);
        return group::is_contains(&ag, c);
    }
    true
}
// function s.cbop(e,tp,eg,ep,ev,re,r,rp)
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    let att = duel::get_attacker(d).nil();
    if card::is_relate_to_effect(d, c, a.e) && !card::is_immune_to_effect(d, att, a.e) {
        // Duel.ChangeAttackTarget(c)
        duel::change_attack_target(d, Some(c), false);
    }
}
