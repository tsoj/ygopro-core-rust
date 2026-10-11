//! Earthbound Immortal Chacu Challhua
use super::prelude::*;

pub const ID: u32 = 69931927;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can only be 1 "Earthbound Immortal" on the field
    // c:SetUniqueOnField(1,1,aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL),LOCATION_MZONE)
    card::set_unique_on_field(
        d,
        c,
        1,
        1,
        card::UniqueCode::Function(lua_fn(|d, a: Args| card::is_set_card(d, a.c(0), &[SET_EARTHBOUND_IMMORTAL as u16]))),
        Some(LOCATION_MZONE),
    );
    // Destroy itself if there is no face-up field spell
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e1, Some(condition(sdcon)));
    card::register_effect(d, c, e1, false);
    // Opponent cannot target this card for attacks
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, aux::imval2());
    card::register_effect(d, c, e2, false);
    // Can attack directly
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    card::register_effect(d, c, e3, false);
    // Inflict damage equal to half of this card's DEF
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_cost(d, e4, Some(cost_target(damcost)));
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
    // Opponent cannot conduct their battle phase while this card is in defense position
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e5, EFFECT_CANNOT_BP);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_target_range(d, e5, 0, 1);
    effect::set_condition(d, e5, Some(condition(bpcon)));
    card::register_effect(d, c, e5, false);
}
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // Cannot attack this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3206);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
    true
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let dam = lua_int(card::get_defense(d, c) as f64 / 2.0);
    duel::set_target_player(d, 1 - tp);
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam as i32);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn bpcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_position(d, c, POS_FACEUP_DEFENSE)
}
