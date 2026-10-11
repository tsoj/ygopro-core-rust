//! Djinn Prognosticator of Rituals
use super::prelude::*;

pub const ID: u32 = 4141820;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be used for a Ritual Summon from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_EXTRA_RITUAL_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_RITUAL_MATERIAL);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Provide an effect to a Ritual monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_none()
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_RITUAL
    a.r == REASON_RITUAL
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, eg) = (a.e, a.eg.clone().nil());
    // for rc in eg:Iter() do
    for rc in group::iter(d, &eg) {
        // if rc:GetFlagEffect(id)==0 then
        if card::get_flag_effect(d, rc, ID) == 0 {
            // --Make the opponent discard 1 card
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e1, aux::stringid(ID, 0));
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_BATTLE_DAMAGE)
            effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetLabel(ep)
            effect::set_label(d, e1, &[a.ep as i64]);
            // e1:SetCondition(s.hdcon)
            effect::set_condition(d, e1, Some(condition(hdcon)));
            // e1:SetOperation(s.hdop)
            effect::set_operation(d, e1, Some(operation(hdop)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // rc:RegisterEffect(e1,true)
            card::register_effect(d, rc, e1, true);
            // rc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, rc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.hdcon(e,tp,eg,ep,ev,re,r,rp)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // return ep==1-e:GetLabel() and eg:GetFirst()==e:GetHandler()
    let label = effect::get_label(d, e);
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, e).nil();
    a.ep as i64 == 1 - label &&group::get_first(d, &eg) == Some(h)
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.DiscardHand(1-e:GetLabel(),nil,1,1,REASON_EFFECT|REASON_DISCARD)
    let p = (1 - effect::get_label(d, e)) as u8;
    duel::discard_hand(d, p, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
