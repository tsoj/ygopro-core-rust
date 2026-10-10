//! Relinquished
use super::prelude::*;

pub const ID: u32 = 64631466;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41426869], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(eqcon)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqcon)),
        lua_fn(|d, a| card::is_controler(d, a.c(0), 1 - a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // Change ATK/DEF
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    effect::set_condition(d, e2, Some(condition(adcon)));
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_SET_DEFENSE);
    effect::set_condition(d, e3, Some(condition(adcon)));
    effect::set_value_fn(d, e3, effect_card_value(defval));
    card::register_effect(d, c, e3, false);
    // damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_DAMAGE);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(damcon)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetEquipGroup():Filter(s.eqfilter,nil); return #g==0
    let c = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, c);
    group::filter(d, &eg, &eqfilter, NO_EX).is_empty()
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_change_controler(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    // substitute
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_DESTROY_SUBSTITUTE);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e2, lua_fn(repval));
    card::register_effect(d, tc, e2, false);
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) && card::is_controler(d, tc, 1 - a.tp) && eqcon(d, a) {
            equipop(d, c, a.e, a.tp, tc);
        }
    }
}
fn repval(_d: &mut Duel, a: Args) -> bool {
    // return r&REASON_BATTLE~=0
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=c:GetEquipGroup():Filter(s.eqfilter,nil)
    // return #g>0 and ep==tp and r&REASON_BATTLE~=0 and (Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, c);
    let g = group::filter(d, &eg, &eqfilter, NO_EX);
    !g.is_empty()
        && a.ep == a.tp
        && (a.r & REASON_BATTLE) != 0
        && (duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c))
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,ev,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, a.ev as i64, REASON_EFFECT, false, None);
}
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, c);
    !group::filter(d, &eg, &eqfilter, NO_EX).is_empty()
}
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let c = effect::get_handler(d, e).nil();
    let eg = card::get_equip_group(d, c);
    let g = group::filter(d, &eg, &eqfilter, NO_EX);
    let first = group::get_first(d, &g).nil();
    let atk = card::get_text_attack(d, first);
    if card::is_facedown(d, first) || (card::get_original_type(d, first) & TYPE_MONSTER) == 0 || atk < 0 {
        0
    } else {
        atk as i64
    }
}
fn defval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let c = effect::get_handler(d, e).nil();
    let eg = card::get_equip_group(d, c);
    let g = group::filter(d, &eg, &eqfilter, NO_EX);
    let first = group::get_first(d, &g).nil();
    let def = card::get_text_defense(d, first);
    if card::is_facedown(d, first) || (card::get_original_type(d, first) & TYPE_MONSTER) == 0 || def < 0 {
        0
    } else {
        def as i64
    }
}
