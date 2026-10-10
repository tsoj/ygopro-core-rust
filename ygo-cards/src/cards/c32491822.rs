//! Hamon, Lord of Striking Thunder
use super::prelude::*;

pub const ID: u32 = 32491822;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Cannot be special summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux_false());
    card::register_effect(d, c, e1, false);
    // Special summon procedure
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    card::register_effect(d, c, e2, false);
    // Inflict Damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
    // Limit batlle target
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_condition(d, e4, Some(condition(atcon)));
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    card::register_effect(d, c, e4, false);
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_continuous_spell(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    spfilter(d, c) || (card::is_facedown(d, c) && card::is_spell(d, c) && card::is_able_to_grave_as_cost(d, c))
}
// The cards that can be sent to the GY as the cost (54828837: a player effect that also allows face-down Spells)
fn cost_group(d: &mut Duel, tp: u8) -> CardSet {
    if duel::is_player_affected_by_effect(d, tp, 54828837).is_some() {
        duel::get_matching_group(d, Some(&exfilter), tp, LOCATION_ONFIELD, 0, NO_EX)
    } else {
        duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_ONFIELD, 0, NO_EX)
    }
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    (aux::chkf_mmz(1)(d, sg, e, tp, mg, c).0, false)
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let g = cost_group(d, tp);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && g.len() > 2
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let g = cost_group(d, tp);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    let dg = group::filter(d, &sg, &|d, c| card::is_facedown(d, c), NO_EX);
    if !dg.is_empty() {
        duel::confirm_cards(d, 1 - tp, dg);
    }
    if sg.len() == 3 {
        effect::set_label_object(d, e, Value::Group(sg));
        return true;
    }
    false
}
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bc = card::get_battle_target(d, c).nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_reason(d, bc, REASON_BATTLE) && card::is_monster(d, bc)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    c != effect::get_handler(d, e).nil()
}
