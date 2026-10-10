//! Number S39: Utopia the Lightning
use super::prelude::*;

pub const ID: u32 = 56832966;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], xyz_number: Some(39), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 5 LIGHT monsters OR 1 Rank 4 "Utopia" monster you control
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(5),
        3,
        Some(xyz::alter_filter(|d, x, tp, xyzc| ovfilter(d, x, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Cannot be used as material for an Xyz Summon
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    effect::set_value(d, e0, 1);
    card::register_effect(d, c, e0, false);
    // If this card battles, your opponent cannot activate cards or effects until the end of the Damage Step
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_condition(d, e1, Some(condition(battle_cond)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // This card's ATK becomes 5000 during that damage calculation only
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // Cost.AND(Cost.DetachFromSelf(2),Cost.SoftOncePerBattle)
    // Cost.SoftOncePerBattle is used without arguments: its function(flag,ct) receives the
    // effect and player as flag and ct and returns a function without checking anything, so
    // it always passes and records nothing.
    effect::set_cost(d, e2, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(2), None, None), cost_no_op()])));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn cost_no_op() -> LuaFn {
    cost_target(|_d, _a| true)
}
/// `ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    card::is_rank(d, c, &[4])
        && card::is_set_card_ctx(d, c, &[SET_UTOPIA as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
fn atkconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_UTOPIA as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `function(e) local c=e:GetHandler() return Duel.GetAttacker()==c or Duel.GetAttackTarget()==c end`
fn battle_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_controler(d, bc, 1 - a.tp) && {
            let og = card::get_overlay_group(d, c);
            group::is_exists(d, &og, &|d, x| atkconfilter(d, x), 1, NO_EX)
        },
        None => false,
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // This card's ATK becomes 5000 during that damage calculation only
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 5000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        card::register_effect(d, c, e1, false);
    }
}
