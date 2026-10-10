//! Alchemic Magician
use super::prelude::*;

pub const ID: u32 = 65301952;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),4,3)
    xyz::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_SPELLCASTER, sc, st, Some(tp)))), Some(4), 3, None, None, None, None, false, None);
    // This card gains 200 ATK for each Spell in your GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // Set 1 Spell from your Deck to your Spell & Trap Zone
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(lua_fn(turncon)));
    effect::set_cost(d, e2, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(setcost)])));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn turncon(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, a.p(1))
}
/// `function(e,c) return Duel.GetMatchingGroupCount(Card.IsSpell,c:GetControler(),LOCATION_GRAVE,0,nil)*200 end`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&card::is_spell), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 200
}
/// `s.setcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and not c:IsFieldSpell() and c:IsSSetable()
    card::is_spell(d, c) && !card::is_field_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SSet(tp,g) end
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}
