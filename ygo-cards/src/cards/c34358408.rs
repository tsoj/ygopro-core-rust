//! Djinn Presider of Rituals
use super::prelude::*;

pub const ID: u32 = 34358408;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be used for a Ritual Summon while it is in the GY
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
    // Grants an effect if it is used for a Ritual Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(bm_condition)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(bm_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    let p = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, p, CARD_SPIRIT_ELIMINATION).is_none()
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn bm_condition(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_RITUAL
    a.r == REASON_RITUAL
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn bm_operation(d: &mut Duel, a: &EffArgs) {
    // for rc in eg:Iter() do
    let eg = a.eg();
    for rc in group::iter(d, &eg) {
        // if rc:GetFlagEffect(id)==0 then
        if card::get_flag_effect(d, rc, ID) == 0 {
            // --Draw 1 card when a monster is destroyed by battle
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e1, aux::stringid(ID, 0));
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_BATTLE_DESTROYING)
            effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
            // e1:SetOperation(s.drawop)
            effect::set_operation(d, e1, Some(operation(drawop)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // rc:RegisterEffect(e1,true)
            card::register_effect(d, rc, e1, true);
            // rc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, rc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.drawop(e,tp,eg,ep,ev,re,r,rp)`
fn drawop(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
