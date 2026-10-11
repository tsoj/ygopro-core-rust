//! DoSolfachord Coolia
use super::prelude::*;

pub const ID: u32 = 80776622;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // act limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(sucop)));
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // negate
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_CHAINING);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn sucfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_controler(d, c, tp) && card::is_pendulum_summoned(d, c)
}
fn sucop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| sucfilter(d, c, a.tp), 1, NO_EX) {
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
}
fn chainlm(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.p(1);
    let tp = a.p(2);
    tp == rp || (effect::is_spell_trap_effect(d, e) && !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE))
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| card::is_type(d, x, TYPE_PENDULUM)),
        2,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(2), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, ..Default::default() },
        NO_EX,
        false,
    )
}
// The target gets the number 1 as its `c` (no chk parameter in the Lua): no card for `to_check`.
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::select_release_group(
        d,
        a.tp,
        Some(&|d, x| card::is_type(d, x, TYPE_PENDULUM)),
        2,
        2,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: None, ..Default::default() },
        NO_EX,
        false,
    );
    if let Some(g) = g {
        // g:KeepAlive()
        effect::set_label_object(d, a.e, Value::Group(g));
        return true;
    }
    false
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    duel::release(d, g, REASON_COST, None);
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let ct: u16 = if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_odd_scale(d, c)), tp, LOCATION_PZONE, 0, 1, NO_EX) { 2 } else { 1 };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable(d, c)), tp, LOCATION_ONFIELD, 0, 1, ct, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e);
    let g = duel::get_target_cards(d, a.e).nil();
    for tc in group::iter(d, &g) {
        if ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER)) && card::is_relate_to_effect(d, tc, a.e) {
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            let e1 = effect::create_effect(d, c.nil());
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c.nil());
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
            card::register_effect(d, tc, e2, false);
            if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                let e2 = effect::create_effect(d, c.nil());
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e2, EFFECT_DISABLE_TRAPMONSTER);
                effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
                card::register_effect(d, tc, e2, false);
            }
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)), tp, LOCATION_PZONE, 0, NO_EX);
    let Some((_, sc)) = group::get_max_group(d, &g, &|d, c| card::get_scale(d, c) as i64) else { return false };
    if card::is_status(d, effect::get_handler(d, a.e).nil(), STATUS_BATTLE_DESTROYED) {
        return false;
    }
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rc = rc.nil();
    card::is_attack_below(d, rc, (sc * 300) as i32) && card::is_on_field(d, rc) && card::is_destructable(d, rc, None)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) {
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
