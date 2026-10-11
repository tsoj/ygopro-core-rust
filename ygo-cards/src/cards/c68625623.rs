//! Unchained Malevolent Magistrate Kamura
use super::prelude::*;

pub const ID: u32 = 68625623;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can target up to 3 cards you control; destroy them, and if you do, Special Summon this card from your hand, then you can apply any of the following effects, in sequence, based on the original types of the destroyed cards
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW + CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card on the field is destroyed by card effect: You can destroy all monsters on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && duel::get_m_zone_count(d, tp, chkc.into(), None, None, None).0 > 0;
    }
    let c = effect::get_handler(d, e).nil();
    let mmz_rescon = aux::chkf_mmz(1);
    let g = duel::get_target_group(d, None, tp, LOCATION_ONFIELD, 0, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty()
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(3), Some(&mmz_rescon), 0, tp, None, None, None, false), Err(true))
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(3), Some(&mmz_rescon), 1, tp, Some(HINTMSG_DESTROY as u64), Some(&mmz_rescon), None, false)
        .unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg.clone()), tg.len() as u8, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    if group::is_exists(d, &tg, &|d, x| card::is_monster_card(d, x), 1, NO_EX) {
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    if group::is_exists(d, &tg, &|d, x| card::is_spell_card(d, x), 1, NO_EX) {
        duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    }
    if group::is_exists(d, &tg, &|d, x| card::is_trap_card(d, x), 1, NO_EX) {
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    }
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tg = duel::get_target_cards(d, e).nil();
    if tg.is_empty() || duel::destroy(d, tg, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    let og = duel::get_operated_group(d);
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Monster: Draw 1 card
        if group::is_exists(d, &og, &|d, x| card::is_monster_card(d, x), 1, NO_EX) && duel::is_player_can_draw(d, tp, 1) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::break_effect(d);
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
        // Spell: Change the battle position of 1 monster on the field
        if group::is_exists(d, &og, &|d, x| card::is_spell_card(d, x), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_can_change_position(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_can_change_position(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::hint_selection(d, g.clone(), true);
                duel::break_effect(d);
                duel::change_position(
                    d,
                    g,
                    POS_FACEUP_DEFENSE as u8,
                    Some(POS_FACEUP_ATTACK as u8),
                    Some(POS_FACEUP_ATTACK as u8),
                    Some(POS_FACEUP_ATTACK as u8),
                    false,
                );
            }
        }
        // Trap: Destroy 1 other card on the field
        if group::is_exists(d, &og, &|d, x| card::is_trap_card(d, x), 1, NO_EX)
            && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
            && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
            if !g.is_empty() {
                duel::hint_selection(d, g.clone(), true);
                duel::break_effect(d);
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_EFFECT)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) > 0;
    }
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
