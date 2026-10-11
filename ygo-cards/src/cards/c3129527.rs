//! Sleepy Scapegoats
use super::prelude::*;

pub const ID: u32 = 3129527;
const TOKEN_SCAPEGOAT: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SCAPEGOAT, 77482666, CARD_DARK_TIME_WIZARD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon up to 4 "Scapegoat Tokens" (Beast/EARTH/Level 1/ATK 0/DEF 0) in Defense Position, then if your opponent controls a monster, you can Special Summon 1 "Swift Panther Warrior" from your Deck, also for the rest of this turn, these Tokens cannot be Tributed for a Tribute Summon, also you cannot Special Summon from the Extra Deck, except Fusion Monsters. If a card(s) you control that mentions "Dark Time Wizard" would be destroyed by battle or card effect, you can destroy 1 Token you control that was Special Summoned by this effect instead
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_SCAPEGOAT, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(a.tp), 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[77482666]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut mmz_count = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if mmz_count > 0 && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SCAPEGOAT, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(tp), 0) {
        mmz_count = mmz_count.min(4);
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            mmz_count = 1;
        }
        if mmz_count > 1 {
            mmz_count = duel::announce_number_range(d, tp, Some(1), Some(mmz_count as u32), &[]).nil().0 as i32;
        }
    }
    let c = effect::get_handler(d, a.e).nil();
    for _ in 1..=mmz_count {
        let token = duel::create_token(d, tp, TOKEN_SCAPEGOAT).nil();
        if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // For the rest of this turn, these Tokens cannot be Tributed for a Tribute Summon
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, 3304);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, token, e1, true);
        }
    }
    if duel::special_summon_complete(d) > 0 {
        // If a card(s) you control that mentions "Dark Time Wizard" would be destroyed by battle or card effect, you can destroy 1 Token you control that was Special Summoned by this effect instead
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
        effect::set_target(d, e2, Some(cost_target(reptg)));
        effect::set_operation(d, e2, Some(operation(repop)));
        effect::set_value_fn(d, e2, effect_card_fn(|d, e, cc| {
            let p = effect::get_handler_player(d, e);
            repfilter(d, cc, p)
        }));
        duel::register_effect(d, e2, tp);
        if duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    // You cannot Special Summon from the Extra Deck, except Fusion Monsters
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_fusion_monster(d, cc))));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e3, tp);
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD])
        && card::is_on_field(d, c)
        && card::is_controler(d, c, tp)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_original_code(d, c, &[TOKEN_SCAPEGOAT])
        && card::is_owner(d, c, tp)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, cc| repfilter(d, cc, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, cc| desfilter(d, cc, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    if duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| desfilter(d, cc, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g).nil();
        duel::hint_selection(d, sc, true);
        effect::set_label_object(d, a.e, Value::Card(sc));
        card::set_status(d, sc, STATUS_DESTROY_CONFIRMED, true);
        return true;
    }
    false
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    card::set_status(d, sc, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, sc, REASON_EFFECT | REASON_REPLACE, None, None);
}
