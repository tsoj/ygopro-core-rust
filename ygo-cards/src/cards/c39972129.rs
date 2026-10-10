//! No.64 Ronin Raccoon Sandayu
use super::prelude::*;

pub const ID: u32 = 39972129;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[39972130], xyz_number: Some(64), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_BEAST, sc, st, Some(tp)))),
        Some(2),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    card::enable_revive_limit(d, c);
    // Special Summon 1 Token
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle or effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e2, Some(lua_fn(indcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(-2),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        let token = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // local g,atk=...:GetMaxGroup(Card.GetAttack); if not g then atk=0 end
        let atk = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map_or(0, |(_, atk)| atk);
        // Set the Token's ATK
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, token, e1, false);
        duel::special_summon_complete(d);
    }
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST))), p, LOCATION_MZONE, 0, 1, h.into())
}
