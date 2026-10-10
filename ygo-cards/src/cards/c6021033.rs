//! Doomkaiser Dragon
use super::prelude::*;

pub const ID: u32 = 6021033;

pub const INFO: ScriptInfo = ScriptInfo { material: &[33420078], listed_names: &[33420078], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(
        d,
        c,
        Some(aux::filter_summon_code(&[33420078])),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_ZOMBIE, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e2, Some(operation(desop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_race(d, tc, RACE_ZOMBIE)
        && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) != 0
    {
        if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        card::set_card_target(d, c, tc);
        effect::set_label_object(d, a.e, Value::Card(tc));
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESET_TURN_SET | RESET_TOFIELD, 0, 0, 0, 0);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    if let Some(tc) = tc {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, tc, ID) != 0 && card::get_flag_effect(d, h, ID) != 0 {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
