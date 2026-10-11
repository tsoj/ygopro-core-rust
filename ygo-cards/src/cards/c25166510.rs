//! Salamangreat Beat Bison
//! scripted by Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 25166510;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon from hand, ignition effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);

    // Return FIRE links to negate, ignition effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SALAMANGREAT],
    ..ScriptInfo::DEFAULT
};

// Defining what to check
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
}

// Check for 3+ "Salamangreat" monsters in your GY
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 3, NO_EX)
}

// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}

// Perfoming the effect of special summoning this card in defense position
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}

// Check for FIRE link monsters
fn filter1(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_GRAVE) && card::is_link_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_extra(d, c)
}

// Activation legality
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter1(d, chkc);
    }
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&filter1), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_GRAVE, 0, 1, ct as u16, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}

// Performing the effect of returning FIRE links, then negating opponent's cards
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let tg = duel::get_target_cards(d, e).nil();
    if !tg.is_empty() && duel::send_to_deck(d, tg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        let ct = duel::get_operated_group(d).len();
        let cg = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
        if ct > 0 && !cg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            let sg = group::select(d, &cg, tp, 1, ct as u16, false, NO_EX).nil();
            duel::break_effect(d);
            for tc in group::iter(d, &sg) {
                duel::negate_related_chain(d, tc, RESET_TURN_SET);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e1, EFFECT_DISABLE);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, tc, e1, false);

                let e2 = effect::create_effect(d, c);
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                effect::set_value(d, e2, RESET_TURN_SET as i64);
                effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, tc, e2, false);

                if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                    let e3 = effect::create_effect(d, c);
                    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
                    effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                    card::register_effect(d, tc, e3, false);
                }
            }
        }
    }
}
