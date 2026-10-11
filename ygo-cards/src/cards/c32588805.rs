//! The Despair Uranus
use super::prelude::*;

pub const ID: u32 = 32588805;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent declares either Continuous Spell or Continuous Trap, then you Set 1 card of that type from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    effect::set_target(d, e1, Some(cost_target(settg)));
    effect::set_operation(d, e1, Some(operation(setop)));
    card::register_effect(d, c, e1, false);
    // This card gains 300 ATK for each face-up Spell/Trap you control
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(e2_value));
    card::register_effect(d, c, e2, false);
    // Face-up cards in your Spell & Trap Zone cannot be destroyed by card effects
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_STZONE, 0);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_faceup(d, c))));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp) return e:GetHandler():IsTributeSummoned() and not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,nil) end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let tp = a.p(1);
    card::is_tribute_summoned(d, h) && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_continuous_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,...,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,...)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let op = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let sg = if op == 0 {
        group::filter_select(d, &g, tp, &|d, c| card::is_continuous_spell(d, c), 1, 1, false, NO_EX).nil()
    } else {
        group::filter_select(d, &g, tp, &|d, c| card::is_continuous_trap(d, c), 1, 1, false, NO_EX).nil()
    };
    if !sg.is_empty() {
        duel::s_set(d, tp, &sg, None, true);
    }
}
/// `function(e,c) return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSpellTrap),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)*300 end`
fn e2_value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    let f = aux::faceup_filter(|d, c| card::is_spell_trap(d, c));
    duel::get_matching_group_count(d, Some(&f), p, LOCATION_ONFIELD, 0, NO_EX) as i64 * 300
}
