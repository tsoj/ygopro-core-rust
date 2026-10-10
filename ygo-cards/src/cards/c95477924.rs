//! Magician's Salvation
use super::prelude::*;

pub const ID: u32 = 95477924;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[48680970, CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Dark Magician" or "Dark Magician Girl" from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    // local g=Group.CreateGroup() g:KeepAlive() e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    card::register_effect(d, c, e2, false);
    // Register the summons of 1 "Dark Magician" or "Dark Magician Girl"
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_condition(d, e3, Some(condition(regcon)));
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[48680970]) && card::is_s_setable(d, c, false, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::s_set(d, tp, sg, None, true);
    }
}
fn regcon(d: &mut Duel, _a: &EffArgs) -> bool {
    let ph = duel::get_current_phase(d);
    ph < PHASE_DAMAGE || ph > PHASE_DAMAGE_CAL
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let code = card::get_code(d, c)[0];
    card::is_faceup(d, c)
        && card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL])
        && card::is_summon_player(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| cfilter(d, c, e, tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN | RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
        let e2 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        group::add_card(&mut g, &tg);
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2, Value::Group(g));
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let stored = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &stored, &|d, c| cfilter(d, c, e, tp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && cfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty();
    }
    if g.len() == 1 {
        let tc = group::get_first(d, &g).nil();
        duel::set_target_card(d, tc);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::set_target_card(d, tc);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            let code = card::get_code(d, tc)[0];
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, code)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
