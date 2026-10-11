//! Salamangreat Zebroid X
use super::prelude::*;

pub const ID: u32 = 44161893;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 "Salamangreat" monsters from your GY and Xyz Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Grant an effect when used as Xyz Material
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_condition(d, e2, Some(condition(efcon)));
    effect::set_operation(d, e2, Some(operation(efop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::get_previous_type_on_field(d, c) & TYPE_LINK as u32 > 0
        && card::is_previous_set_card(d, c, &[SET_SALAMANGREAT as u16])
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::get_reason_player(d, c) == 1 - tp
        && card::is_reason(d, c, REASON_EFFECT)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, c) && group::is_exists(d, &eg, &|d, cc| cfilter(d, cc, a.tp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
        && card::is_level(d, c, &[4])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && {
            let mut mg = CardSet::from_card(c);
            group::add_card(&mut mg, &CardSet::from_card(tc));
            duel::is_existing_matching_card(d, Some(&|d, cc| xyzfilter(d, cc, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 3, tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() || !duel::is_player_can_special_summon_count(d, tp, 2) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let mut sg = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    if sg.is_empty() {
        return;
    }
    group::add_card(&mut sg, &CardSet::from_card(c));
    for sc in group::iter(d, &sg) {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Negate their effects
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, sc, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            card::register_effect(d, sc, e2, false);
        }
    }
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    duel::break_effect(d);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let xg = duel::select_matching_card(d, tp, Some(&|d, cc| xyzfilter(d, cc, &sg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(xyz) = group::get_first(d, &xg) {
        duel::xyz_summon(d, tp, xyz, Some(sg.clone()), None, 2, 2);
    }
}
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    (a.r & REASON_XYZ) > 0 && {
        let c = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, c, LOCATION_ONFIELD)
    }
}
fn efop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    // Increase ATK
    let e1 = effect::create_effect(d, rc);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atk_val));
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    card::register_effect(d, rc, e1, true);
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // Treated as an Effect Monster
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, rc, e2, true);
    }
}
/// `function(_,c) return c:GetOverlayCount()*300 end`
fn atk_val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_overlay_count(d, c) as i64 * 300
}
