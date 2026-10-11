//! Nightmare-Eyes Restrict
use super::prelude::*;

pub const ID: u32 = 30138615;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 monsters with different Types (Fiend, Illusion, or Spellcaster)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| matfilter(d, c, a))), 2)]);
    // Equip 1 monster your opponent controls to this card as an Equip Spell
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_EQUIP);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(eqtg)));
    effect::set_operation(d, e1a, Some(operation(eqop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1b, EVENT_PHASE + PHASE_BATTLE);
    effect::set_range(d, e1b, LOCATION_MZONE);
    effect::set_condition(d, e1b, Some(condition(battled)));
    card::register_effect(d, c, e1b, false);
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_args), e1a, None, None, None);
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_args), e1b, None, None, None);
    // Gains ATK equal to the combined ATK of those equipped monsters
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    card::register_effect(d, c, e2, false);
    // If this card battles a monster, neither can be destroyed by that battle
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // Your opponent's monsters cannot attack monsters, except this one
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e4, effect_card_fn(e4_value));
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c,fc,sumtype,sp,sub,mg,sg)`
fn matfilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    if !card::is_race_ctx(d, c, RACE_FIEND | RACE_ILLUSION | RACE_SPELLCASTER, Some(a.fc), a.sumtype, Some(a.tp)) {
        return false;
    }
    let Some(sg) = a.sg else { return true };
    let race = card::get_race(d, c);
    group::filter_count(d, sg, &|_d: &mut Duel, _x: CardId| true, c.into()) == 0
        || !group::is_exists(d, sg, &|d: &mut Duel, x: CardId| card::is_race_ctx(d, x, race, Some(a.fc), a.sumtype, Some(a.tp)), 1, c.into())
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    let tp = a.p(2);
    card::is_controler(d, ec, 1 - tp)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
}
fn equipop_args(d: &mut Duel, a: Args) -> bool {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
    true
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, cc| card::is_able_to_change_controler(d, cc)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sel) {
        duel::hint_selection(d, CardSet::from_card(sc), true);
        equipop(d, c, a.e, a.tp, sc);
    }
}
fn battled(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `e:GetHandler():GetEquipGroup():IsExists(aux.FaceupFilter(Card.HasFlagEffect,id),1,nil)`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, h);
    group::is_exists(d, &eg, &aux::faceup_filter(|d, c| card::has_flag_effect(d, c, ID, None)), 1, NO_EX)
}
/// `function(e,c) return c:GetEquipGroup():Match(...):GetSum(Card.GetTextAttack) end`
fn e2_value(d: &mut Duel, a: Args) -> i64 {
    let c = a.c(1);
    let eg = card::get_equip_group(d, c);
    let mut m = eg.clone();
    group::match_(d, &mut m, &|d: &mut Duel, x: CardId| card::has_flag_effect(d, x, ID, None) && card::is_faceup(d, x) && card::get_text_attack(d, x) > 0, NO_EX);
    group::get_sum(d, &m, &|d: &mut Duel, x: CardId| card::get_text_attack(d, x) as i64)
}
/// `function(e,c) local handler=e:GetHandler() return c==handler or c==handler:GetBattleTarget() end`
fn e3_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c == h || Some(c) == card::get_battle_target(d, h)
}
/// `function(e,c) return c~=e:GetHandler() end`
fn e4_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    c != effect::get_handler(d, e).nil()
}
