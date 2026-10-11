//! Megalith Notrah Plura
use super::prelude::*;

pub const ID: u32 = 44293356;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    mat_filter: Some(mat_filter),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    card::add_must_be_ritual_summoned(d, c);
    // Apply a "this turn, your opponent cannot activate cards or effects in response to the activation of your "Megalith" Ritual Monsters' effects" effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1cond)));
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's card or effect, and if you do, destroy that card, then if that effect targeted a card(s) on the field, you can Tribute 1 monster your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_RELEASE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2cond)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
fn mat_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::is_ritual_monster(d, c)
}
fn e1cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::has_flag_effect(d, a.tp, ID, None)
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    let c = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // This turn, your opponent cannot activate cards or effects in response to the activation of your "Megalith" Ritual Monsters' effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_operation(d, e1, Some(operation(inactop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
fn inactop(d: &mut Duel, a: &EffArgs) {
    let ch = a.ev as u8;
    if a.ep as i64 == a.tp as i64
        && chain::is_triggering(d, ch, chain::Prop::SetCard, &[SET_MEGALITH as i64]).truthy()
        && chain::is_composite_type(d, ch, (TYPE_RITUAL | TYPE_MONSTER) as u32).truthy()
    {
        duel::set_chain_limit(d, lua_fn(|_d, a: Args| a.int(1) == a.int(2)));
    }
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned();
    let has_field_target = match tg {
        Some(tg) => group::is_exists(d, &tg, &|d, c| card::is_on_field(d, c), 1, NO_EX),
        None => false,
    };
    if has_field_target {
        effect::set_label(d, a.e, &[100]);
        duel::set_possible_operation_info(d, 0, CATEGORY_RELEASE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
    true
}
fn e2cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let opponent = a.ep as i64 == 1 - a.tp as i64;
    opponent && !card::is_status(d, c, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
        let label = effect::get_label(d, a.e);
        if label == 100
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            if g.len() > 0 {
                duel::hint_selection(d, g.clone(), true);
                duel::break_effect(d);
                duel::release(d, g, REASON_EFFECT, None);
            }
        }
    }
}
