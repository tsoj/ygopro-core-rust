//! Black Garden
use super::prelude::*;

pub const ID: u32 = 71645242;
const SUMMONED_BY_BLACK_GARDEN: u32 = 0x20;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, TOKEN_ROSE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Halve the ATK of a Summoned monster, then, its controller Special Summons 1 "Rose Token" to their opponent's field in Attack Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    effect::set_target(d, e1, Some(cost_target(atksptg)));
    effect::set_operation(d, e1, Some(operation(atkspop)));
    card::register_effect(d, c, e1, false);
    // Destroy this card and as many Plant monsters as possible, then Special Summon 1 monster in your GY with ATK equal to the total ATK of all Plant monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target(d, e2, Some(cost_target(dessptg)));
    effect::set_operation(d, e2, Some(operation(desspop)));
    card::register_effect(d, c, e2, false);
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        effect::set_condition(d, ge1, Some(condition(regcon)));
        effect::set_operation(d, ge1, Some(operation(regop)));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::clone(d, ge1);
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        duel::register_effect(d, ge2, 0);
    });
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::get_summon_type(d, c) != SUMMON_TYPE_SPECIAL + SUMMONED_BY_BLACK_GARDEN
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let mut sf = 0;
    if group::is_exists(d, &eg, &|d, c| cfilter(d, c, 0), 1, NO_EX) {
        sf += 1;
    }
    if group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1), 1, NO_EX) {
        sf += 2;
    }
    effect::set_label(d, a.e, &[sf]);
    sf != 0
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    duel::raise_event(d, a.eg(), EVENT_CUSTOM + ID, Some(a.e), a.r, a.rp, a.ep, label as u32);
}
fn atksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, h, a.e);
    }
    duel::set_target_card(d, a.eg());
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    let tp = a.tp;
    let ev = a.ev;
    let p: u8 = if (ev >> tp) & 1 != 0 && (ev >> (1 - tp)) & 1 != 0 {
        PLAYER_ALL as u8
    } else if (ev >> tp) & 1 != 0 {
        tp
    } else {
        1 - tp
    };
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, p, 0);
    true
}
fn atkfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) && !card::is_immune_to_effect(d, c, e)
}
fn rose_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ROSE as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(800),
        level: Some(2),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn atkspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| atkfilter(d, c, a.e), NO_EX);
    if g.is_empty() {
        return;
    }
    let mut change = false;
    for tc in group::iter(d, &g) {
        let preatk = card::get_attack(d, tc);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, (atk as f64 / 2.0).ceil() as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        if !card::is_immune_to_effect(d, tc, e1) && (preatk as f64 / 2.0).ceil() as i32 == card::get_attack(d, tc) {
            change = true;
        }
    }
    if !change {
        return;
    }
    if ((a.ev >> tp) & 1) != 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_ROSE, rose_data(), Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), 0)
    {
        let token = duel::create_token(d, tp, TOKEN_ROSE).nil();
        duel::special_summon_step(d, token, SUMMONED_BY_BLACK_GARDEN, tp, 1 - tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
    if ((a.ev >> (1 - tp)) & 1) != 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, 1 - tp, TOKEN_ROSE, rose_data(), Some(POS_FACEUP_ATTACK as u8), Some(tp), 0)
    {
        let token = duel::create_token(d, 1 - tp, TOKEN_ROSE).nil();
        duel::special_summon_step(d, token, SUMMONED_BY_BLACK_GARDEN, 1 - tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
    duel::special_summon_complete(d);
}
fn spfilter(d: &mut Duel, c: CardId, atk: i64, e: EffectId, tp: u8) -> bool {
    card::is_attack(d, c, &[atk as u32]) && card::is_can_be_special_summoned(d, c, e, SUMMONED_BY_BLACK_GARDEN, tp, false, false, None, None, None)
}
fn plant_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)
}
fn dessptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, effect::get_label(d, a.e), a.e, tp);
    }
    let g = duel::get_matching_group(d, Some(&plant_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let atk = group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64);
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, atk, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let tg = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, atk, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    effect::set_label(d, a.e, &[atk]);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, tg, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    let mut g = g;
    group::add_card(&mut g, &CardSet::from_card(h));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn desspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let mut dg = duel::get_matching_group(d, Some(&plant_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    group::add_card(&mut dg, &CardSet::from_card(c));
    duel::destroy(d, dg.clone(), REASON_EFFECT, None, None);
    let mut og = duel::get_operated_group(d);
    if dg.len() != og.len() {
        return;
    }
    let mut ct = 0;
    for oc in group::iter(d, &og) {
        if group::is_contains(&dg, oc) {
            ct += 1;
        }
    }
    if ct != og.len() {
        return;
    }
    duel::break_effect(d);
    group::remove_card(&mut og, &CardSet::from_card(c));
    let _atk = group::get_sum(d, &og, &|d, cc| card::get_previous_attack_on_field(d, cc) as i64);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, CardSet::from_card(tc), SUMMONED_BY_BLACK_GARDEN, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
