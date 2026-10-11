//! Majestic Red Dragon
use super::prelude::*;

pub const ID: u32 = 67030233;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_RED_DRAGON_ARCHFIEND, 21159309],
    listed_names: &[CARD_RED_DRAGON_ARCHFIEND, 21159309], // Majestic Dragon
    synchro_nt_required: Some(1),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    synchro::add_majestic_procedure(
        d,
        c,
        mat_filter(|d, c, _sc, _st, _tp| card::is_code(d, c, &[21159309])),
        true,
        mat_filter(|d, c, _sc, _st, _tp| card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])),
        true,
        synchro::non_tuner(None),
        false,
        vec![],
    );
    // Destroy all Defense Position monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Negate the effects of 1 monster your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Return itself to the Extra Deck and Special Summon 1 "Red Dragon Archfiend"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by effects
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
    // Multiple tuners
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e5, lua_fn(valcheck));
    card::register_effect(d, c, e5, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    h == duel::get_attacker(d)
}
fn is_defense_pos(d: &mut Duel, c: CardId) -> bool {
    card::is_defense_pos(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&is_defense_pos), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&is_defense_pos), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_disabled(d, tc) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        if card::is_immune_to_effect(d, tc, e1) || card::is_immune_to_effect(d, tc, e2) || !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
            return;
        }
        duel::adjust_instantly(d, Some(tc));
        let atk = card::get_attack(d, tc);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e3, atk as i64);
        effect::set_reset(d, e3, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e3, false);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_able_to_extra(d, c)
        && duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, c, LOCATION_EXTRA)
        && tc.is_some_and(|tc| card::is_relate_to_effect(d, tc, a.e))
    {
        duel::special_summon(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_TUNER), 2, NO_EX) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
