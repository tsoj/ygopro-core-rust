//! Firewall Dragon Darkfluid - Neo Tempest Terahertz
use super::prelude::*;

pub const ID: u32 = 64211118;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 3+ Cyberse monsters
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))),
        3,
        None,
        None,
        None,
        None,
    );
    // Negate any monster effects activated by your opponent during the Battle Phase
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_operation(d, e1, Some(operation(e1_op)));
    card::register_effect(d, c, e1, false);
    // Send 1 Cyberse monster from your Deck or Extra Deck to the GY, and if you do, this card gains the Attribute of that monster sent to the GY, also it gains 2500 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // This card can attack monsters a number of times each Battle Phase, up to the number of different Attributes it has.
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK_MONSTER);
    effect::set_value_fn(d, e3, effect_card_value(|d, e, _c| {
        let h = effect::get_handler(d, e).nil();
        group::get_bin_class_count(d, &CardSet::from_card(h), &|d, cc| card::get_attribute(d, cc) as i64) as i64 - 1
    }));
    card::register_effect(d, c, e3, false);
}
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
fn e1_op(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::negate_effect(d, a.ev as u8);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, h.map(CardSet::from_card), 1, tp, 2500);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let sg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, sc, LOCATION_GRAVE)
            && card::is_relate_to_effect(d, c, e)
            && card::is_faceup(d, c)
        {
            // This card gains the Attribute of that monster sent to the GY, also it gains 2500 ATK
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
            effect::set_range(d, e1, LOCATION_MZONE);
            let attr = card::get_attribute(d, sc) as i64;
            effect::set_value(d, e1, attr);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            card::register_effect(d, c, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e2, 2500);
            card::register_effect(d, c, e2, false);
        }
    }
}
