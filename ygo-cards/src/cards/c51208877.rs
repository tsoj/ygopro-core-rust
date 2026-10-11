//! Blast Held by Destiny
use super::prelude::*;

pub const ID: u32 = 51208877;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EXCHANGE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster with the highest ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(cond1)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Add 1 Level 4 EARTH Fairy monster to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(lua_fn(cond2)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn cond1(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, 1 - a.p(1))
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_DECK) end`
fn cond2(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, c, LOCATION_HAND | LOCATION_DECK)
}
fn faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
}
fn max_atk(d: &mut Duel, g: &Group) -> Group {
    group::get_max_group(d, g, &|d: &mut Duel, c: CardId| card::get_attack(d, c) as i64).map(|x| x.0).unwrap_or_default()
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&faceup), a.tp, 0, LOCATION_MZONE, NO_EX);
    let g = max_atk(d, &g);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn is_exchange_spirit(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_EXCHANGE_SPIRIT])
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&faceup), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    let mut tg = max_atk(d, &g);
    if tg.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        tg = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, tg.clone(), true);
    }
    if !tg.is_empty() && duel::destroy(d, tg.clone(), REASON_EFFECT, None, None) > 0 {
        let first = group::get_first(d, &tg).nil();
        let amount = card::get_text_attack(d, first) as i64;
        let dam = duel::damage(d, 1 - a.tp, amount, REASON_EFFECT, false, None);
        if dam > 0 && !duel::is_existing_matching_card(d, Some(&is_exchange_spirit), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            duel::break_effect(d);
            duel::damage(d, a.tp, dam as i64, REASON_EFFECT, false, None);
        }
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[4]) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_FAIRY) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
