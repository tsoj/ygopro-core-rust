//! Unchained Enma Lord Yama
use super::prelude::*;

pub const ID: u32 = 94014327;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UNCHAINED_SOUL], material_setcode: &[SET_UNCHAINED_SOUL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including an "Unchained Soul" monster
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(matcheck)), None, None);
    // During your End Phase: You can target up to 2 Fiend monsters in your GY that were destroyed and sent there this turn; Special Summon them, but you cannot Special Summon, except Fiend monsters, while you control either of those face-up monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card on the field would be destroyed by battle or card effect, you can destroy 1 face-up card on either field instead
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    effect::set_operation(d, e2, Some(operation(desrepop)));
    card::register_effect(d, c, e2, false);
}
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_UNCHAINED_SOUL as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, turn_count: i32) -> bool {
    card::is_race(d, c, RACE_FIEND)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::get_turn_id(d, c) == turn_count as u32
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let turn_count = duel::get_turn_count(d, None);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp, turn_count);
    }
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp, turn_count)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    ft = ft.min(2);
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, turn_count)), tp, LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tg = duel::get_target_cards(d, e).nil();
    if tg.is_empty() || (tg.len() == 2 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    if tg.len() == 2 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // `tg:FilterSelect(...)` only prompts: the Lua does not use its result
        group::filter_select(d, &tg, tp, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), 1, 1, false, NO_EX);
    } else {
        for tc in group::iter(d, &tg) {
            if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
                // You cannot Special Summon, except Fiend monsters, while you control either of those face-up monsters
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                effect::set_range(d, e1, LOCATION_MZONE);
                effect::set_absolute_range(d, e1, tp, 1, 0);
                effect::set_condition(d, e1, Some(lua_fn(|d, a| {
                    let h = effect::get_handler(d, a.e(0)).nil();
                    let owner_player = effect::get_owner_player(d, a.e(0));
                    Value::Bool(card::is_controler(d, h, owner_player))
                })));
                effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_FIEND, None, 0, None))));
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, false);
            }
        }
        duel::special_summon_complete(d);
    }
}
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::is_destructable(d, c, Some(e)) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
        let tc = group::get_first(d, &g).nil();
        duel::hint_selection(d, tc, true);
        effect::set_label_object(d, e, Value::Card(tc));
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
