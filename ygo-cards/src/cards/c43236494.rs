//! Fairy Tale Prologue: Journey's Dawn
use super::prelude::*;

pub const ID: u32 = 43236494;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Any battle damage a player takes is halved
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value(d, e1, HALF_DAMAGE as i64);
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // Place 1 Field Spell from your hand or Deck face-up in your Field Zone, except "Fairy Tale Prologue: Journey's Dawn"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(cond3)));
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(pltg)));
    effect::set_operation(d, e3, Some(operation(plop)));
    card::register_effect(d, c, e3, false);
}
// function(e,tp) return Duel.IsTurnPlayer(tp) end
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
// function s.drconfilter(c)
fn drconfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST)) or (c:IsLevel(7,8) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO))) and c:IsFaceup()
    ((card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_BEAST))
        || (card::is_level(d, c, &[7, 8]) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)))
        && card::is_faceup(d, c)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.drconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&drconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFieldSpell() and not c:IsForbidden() and c:CheckUniqueOnField(tp) and not c:IsCode(id)
    card::is_field_spell(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_code(d, c, &[ID])
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| plfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    if let Some(fc) = fc {
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        duel::break_effect(d);
    }
    // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
    duel::move_to_field(d, sc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
}
