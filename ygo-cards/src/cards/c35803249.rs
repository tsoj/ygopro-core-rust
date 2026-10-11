//! Jinzo - Lord
use super::prelude::*;

pub const ID: u32 = 35803249;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    card::add_must_be_special_summoned(d, c);
    // Special Summon this card by sending 1 face-up "Jinzo" you control to the Graveyard
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(hspcon)));
    effect::set_target(d, e2, Some(lua_fn(hsptg)));
    effect::set_operation(d, e2, Some(lua_fn(hspop)));
    card::register_effect(d, c, e2, false);
    // Trap Cards cannot be activated and the effects of all Trap Cards on the field are negated.
    let e3a = effect::create_effect(d, c);
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3a, EFFECT_CANNOT_TRIGGER);
    effect::set_property(d, e3a, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e3a, LOCATION_MZONE);
    effect::set_target_range(d, e3a, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e3a, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e3a, false);
    // Negate Trap Effects on the field
    let e3b = effect::create_effect(d, c);
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3b, EFFECT_DISABLE);
    effect::set_range(d, e3b, LOCATION_MZONE);
    effect::set_target_range(d, e3b, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3b, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e3b, false);
    let e3c = effect::create_effect(d, c);
    effect::set_type(d, e3c, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3c, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3c, LOCATION_MZONE);
    effect::set_operation(d, e3c, Some(operation(disop)));
    card::register_effect(d, c, e3c, false);
    // Negate Trap Monsters and their effects
    let e3d = effect::create_effect(d, c);
    effect::set_type(d, e3d, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3d, EFFECT_DISABLE_TRAPMONSTER);
    effect::set_range(d, e3d, LOCATION_MZONE);
    effect::set_target_range(d, e3d, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3d, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e3d, false);
    // Once per turn: You can destroy as many face-up Traps on the field as possible, and if you do, inflict 300 damage to your opponent for each card destroyed.
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // Double Snare validity check
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn hspfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_JINZO) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_JINZO]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    let rg = duel::get_matching_group(d, Some(&hspfilter), tp, LOCATION_MZONE, 0, NO_EX);
    !rg.is_empty() && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let _c = effect::get_handler(d, e);
    let rg = duel::get_matching_group(d, Some(&hspfilter), tp, LOCATION_MZONE, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    if tl == LOCATION_SZONE as i64 && effect::is_trap_effect(d, a.re.nil()) {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn trap_card(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let f = aux::faceup_filter(trap_card);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    let sg = duel::get_matching_group(d, Some(&f), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg.clone()), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, sg.len() as i32 * 300);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let f = aux::faceup_filter(trap_card);
    let sg = duel::get_matching_group(d, Some(&f), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let ct = duel::destroy(d, sg, REASON_EFFECT, None, None);
    duel::damage(d, 1 - tp, ct as i64 * 300, REASON_EFFECT, false, None);
}
