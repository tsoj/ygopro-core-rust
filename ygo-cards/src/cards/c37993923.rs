//! Junk Gardna
use super::prelude::*;

pub const ID: u32 = 37993923;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_JUNK_SYNCHRON],
    material_setcode: &[SET_SYNCHRON],
    listed_names: &[CARD_JUNK_SYNCHRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Junk Synchron" + 1+ non-Tuner monsters
    synchro::add_procedure(d, c, Some(mat_filter(tunerfilter)), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Change the battle position of 1 monster your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(postg(true)));
    effect::set_operation(d, e1, Some(posop(true)));
    card::register_effect(d, c, e1, false);
    // Change the battle position of 1 monster on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_POSITION);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_ONFIELD)
    })));
    effect::set_target(d, e2, Some(postg(false)));
    effect::set_operation(d, e2, Some(posop(false)));
    card::register_effect(d, c, e2, false);
}
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    card::is_summon_code(d, c, lc, stype, tp, &[CARD_JUNK_SYNCHRON]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
fn postg(oppo: bool) -> LuaFn {
    cost_target(move |d, a| {
        let tp = a.tp;
        if let Some(chkc) = a.chkc() {
            return (card::is_controler(d, chkc, 1 - tp) || !oppo)
                && card::is_location(d, chkc, LOCATION_MZONE)
                && card::is_can_change_position(d, chkc);
        }
        let location = if oppo { 0 } else { LOCATION_MZONE };
        if a.chk == Some(0) {
            return duel::is_existing_target(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, location, LOCATION_MZONE, 1, NO_EX);
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_can_change_position(d, c)), tp, location, LOCATION_MZONE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, 0);
        true
    })
}
fn posop(oppo: bool) -> LuaFn {
    operation(move |d, a| {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && (card::is_controler(d, tc, 1 - a.tp) || !oppo) {
            duel::change_position(
                d,
                tc,
                POS_FACEUP_DEFENSE as u8,
                Some(POS_FACEDOWN_DEFENSE as u8),
                Some(POS_FACEUP_ATTACK as u8),
                Some(POS_FACEUP_ATTACK as u8),
                false,
            );
        }
    })
}
