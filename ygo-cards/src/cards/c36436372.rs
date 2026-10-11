//! Gimmick Puppet Little Soldiers
use super::prelude::*;

pub const ID: u32 = 36436372;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change this card's Level to the Level of the "Gimmick Puppet" monster sent to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(selflvcost)));
    effect::set_target(d, e1, Some(cost_target(selflvtg)));
    effect::set_operation(d, e1, Some(operation(selflvop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Increase the Levels of up to 2 "Gimmick Puppet" monsters you control by 4
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_LVCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(gplvtg)));
    effect::set_operation(d, e3, Some(operation(gplvop)));
    card::register_effect(d, c, e3, false);
}

fn lvcostfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
        && card::is_monster(d, c)
        && !card::is_level(d, c, &[lv])
        && card::is_able_to_grave_as_cost(d, c)
}

fn selflvcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, h);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| lvcostfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| lvcostfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    let first = group::get_first(d, &g).nil();
    let lvl = card::get_level(d, first) as i64;
    effect::set_label(d, a.e, &[lvl]);
    true
}

fn selflvtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    let lbl = effect::get_label(d, a.e) as i32;
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(h)), 1, a.tp, lbl);
    true
}

fn selflvop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // This card's Level becomes the sent monster's
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e1, effect::get_label(d, a.e));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}

fn lvtgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]) && card::is_faceup(d, c) && card::has_level(d, c)
}

fn gplvtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && lvtgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&lvtgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&lvtgfilter), a.tp, LOCATION_MZONE, 0, 1, 2, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(g.clone()), g.len() as u8, a.tp, 4);
    true
}

fn gplvop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        // Increase its Level by 4
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, 4);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
