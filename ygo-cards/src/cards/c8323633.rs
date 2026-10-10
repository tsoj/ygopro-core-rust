//! Cloak and Dagger
use super::prelude::*;

pub const ID: u32 = 8323633;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // local ac=Duel.AnnounceCard(tp,TYPE_MONSTER)
    let ac = duel::announce_card(d, tp, &[TYPE_MONSTER as u64, OPCODE_ISTYPE as u64]) as u32;
    // c:SetHint(CHINT_CARD,ac)
    card::set_hint(d, c, CHINT_CARD as u8, ac as u64);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // e1:SetLabel(ac)
    effect::set_label(d, e1, &[ac as i64]);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    // c:RegisterEffect(e2)
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    // e3:SetCode(EVENT_FLIP)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_FLIP);
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,code)`
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(code)
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,e:GetLabel()) and rp~=tp
    let code = effect::get_label(d, a.e) as u32;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, code), 1, NO_EX) && a.rp != a.tp
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, c, a.e);
    }
    let code = effect::get_label(d, a.e) as u32;
    let eg = a.eg.clone().nil();
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // local g=eg:Filter(s.filter,nil,e:GetLabel())
    let mut g = group::filter(d, &eg, &|d, x| filter(d, x, code), NO_EX);
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let code = effect::get_label(d, a.e) as u32;
    let eg = a.eg.clone().nil();
    // local g=eg:Filter(s.filter,nil,e:GetLabel()):Filter(Card.IsRelateToEffect,nil,e)
    let g = group::filter(d, &eg, &|d, x| filter(d, x, code), NO_EX);
    let mut g = group::filter(d, &g, &|d, x| card::is_relate_to_effect(d, x, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
