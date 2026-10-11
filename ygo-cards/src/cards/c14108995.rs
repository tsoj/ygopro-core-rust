//! Vernusylph Corolla
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 14108995;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VERNUSYLPH], ..ScriptInfo::DEFAULT };

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
    // EARTH monsters become "Vernusylph" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ADD_SETCODE)
    effect::set_code(d, e2, EFFECT_ADD_SETCODE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_EARTH))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_EARTH))));
    // e2:SetValue(SET_VERNUSYLPH)
    effect::set_value(d, e2, SET_VERNUSYLPH as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // "Vernusylph" cost replacement
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_COST_REPLACE)
    effect::set_code(d, e3, EFFECT_COST_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e3, LOCATION_HAND, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, lua_fn(repval));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(lua_fn(repop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(2)).nil();
    // return c:IsMonster() and c:IsSetCard(SET_VERNUSYLPH) and c:IsDiscardable()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_VERNUSYLPH as u16]) && card::is_discardable(d, c, None, None)
}
/// `s.hintselectionfilter(c)`
fn hintselectionfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and not c:HasFlagEffect(id)
    card::is_code(d, c, &[ID]) && !card::has_flag_effect(d, c, ID, None)
}
/// `s.repop(base,extracon,e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: Args) {
    let tp = a.p(3);
    // local c=base:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // if Duel.IsExistingMatchingCard(s.hintselectionfilter,tp,LOCATION_SZONE,0,1,c) then
    if duel::is_existing_matching_card(d, Some(&hintselectionfilter), tp, LOCATION_SZONE, 0, 1, c.into()) {
        // Duel.HintSelection(c)
        duel::hint_selection(d, c, true);
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,nil,aux.Stringid(id,1))
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST|REASON_DISCARD)
    let eh = effect::get_handler(d, a.e(2)).nil();
    duel::send_to_grave(d, eh, REASON_COST | REASON_DISCARD, None, None);
}
