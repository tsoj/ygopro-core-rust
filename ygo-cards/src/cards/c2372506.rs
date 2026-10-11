//! Chaos Magical Hats
use super::prelude::*;

pub const ID: u32 = 2372506;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a monster effect, or a Normal Spell/Trap Card: ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if a.rp != 1 - a.tp {
        return false;
    }
    effect::is_monster_effect(d, re)
        || (effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
            let h = effect::get_handler(d, re).nil();
            card::is_normal_spell_trap(d, h)
        })
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]) && card::is_can_turn_set(d, c)
}
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: None,
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(0),
        defense: Some(0),
        level: Some(8),
        race: Some(RACE_SPELLCASTER),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_DECK, 0, NO_EX);
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&aux::dncheck()), 0, tp, None, None, None, false), Err(true))
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 3
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), Some(POS_FACEDOWN_DEFENSE as u8), None, 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c)), tp, LOCATION_MMZONE, 0, 1, NO_EX);
    }
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    let mut sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    if sg.len() != 3 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let mg = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c)), tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX).nil();
    let Some(mc) = group::get_first(d, &mg) else { return };
    if card::is_immune_to_effect(d, mc, a.e) {
        return;
    }
    duel::hint_selection(d, mc, true);
    for sc in group::iter(d, &sg) {
        // Special Summon the chosen Spells/Traps as Normal Monsters (Spellcaster/DARK/Level 8/ATK 0/DEF 0), named "Chaos Hats"
        let e1 = effect::create_effect(d, sc);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        effect::set_value(d, e1, (TYPE_NORMAL | TYPE_MONSTER) as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY, 1);
        card::register_effect(d, sc, e1, true);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_CHANGE_RACE);
        effect::set_value(d, e2, RACE_SPELLCASTER as i64);
        card::register_effect(d, sc, e2, true);
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CHANGE_ATTRIBUTE);
        effect::set_value(d, e3, ATTRIBUTE_DARK as i64);
        card::register_effect(d, sc, e3, true);
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e4, 8);
        card::register_effect(d, sc, e4, true);
        let e5 = effect::clone(d, e1);
        effect::set_code(d, e5, EFFECT_SET_BASE_ATTACK);
        effect::set_value(d, e5, 0);
        card::register_effect(d, sc, e5, true);
        let e6 = effect::clone(d, e1);
        effect::set_code(d, e6, EFFECT_SET_BASE_DEFENSE);
        effect::set_value(d, e6, 0);
        card::register_effect(d, sc, e6, true);
        let e7 = effect::clone(d, e1);
        effect::set_code(d, e7, EFFECT_CHANGE_CODE);
        effect::set_value(d, e7, ID as i64);
        card::register_effect(d, sc, e7, true);
    }
    duel::special_summon(d, &sg, 0, tp, tp, true, false, POS_FACEDOWN_DEFENSE as u8, None);
    duel::confirm_cards(d, 1 - tp, &sg);
    if !card::is_has_effect(d, mc, EFFECT_LIGHT_OF_INTERVENTION, None).is_empty() {
        duel::change_position(d, mc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    } else {
        duel::change_position(d, mc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        card::clear_effect_relation(d, mc);
    }
    group::add_card(&mut sg, &CardSet::from_card(mc));
    duel::shuffle_set_card(d, &sg);
    // Your opponent's activated effect becomes "Destroy 1 face-down Defense Position monster your opponent controls"
    duel::break_effect(d);
    let tg = CardSet::new();
    duel::change_target_card(d, a.ev as u8, &tg);
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, &g, true);
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
