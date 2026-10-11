//! BM-4 Blast Spider
use super::prelude::*;

pub const ID: u32 = 40634253;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 DARK Machine monster you control and 1 face-up card your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Inflict damage equal to half the ATK of a destroyed monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(damcon1)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(damcon2)));
    card::register_effect(d, c, e3, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let mut g1 = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g2 = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 2, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.damcon1(e,tp,eg,ep,ev,re,r,rp)`
fn damcon1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    let bc = card::get_battle_target(d, tc);
    !card::is_previous_controler(d, tc, tp)
        && card::is_location(d, tc, LOCATION_GRAVE)
        && card::get_text_attack(d, tc) > 0
        && card::is_controler(d, bc.nil(), tp)
        && card::get_original_attribute(d, bc.nil()) == ATTRIBUTE_DARK
        && card::get_original_race(d, bc.nil()) == RACE_MACHINE
}
/// `s.damfilter1(c,tp)`
fn damfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_monster(d, c)
        && card::get_text_attack(d, c) > 0
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_location(d, c, LOCATION_GRAVE)
        && !card::is_previous_controler(d, c, tp)
}
/// `s.damcon2(e,tp,eg,ep,ev,re,r,rp)`
fn damcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let Some(re) = a.re else { return false };
    let rc = effect::get_handler(d, re).nil();
    let eg = a.eg.clone().nil();
    card::is_controler(d, rc, tp)
        && card::get_original_attribute(d, rc) == ATTRIBUTE_DARK
        && card::get_original_race(d, rc) == RACE_MACHINE
        && group::is_exists(d, &eg, &|d, c| damfilter1(d, c, tp), 1, NO_EX)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.damfilter2(c,tp)`
fn damfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::get_text_attack(d, c) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
        && !card::is_previous_controler(d, c, tp)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| damfilter2(d, c, tp), NO_EX);
    if !g.is_empty() {
        if g.len() > 1 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
            g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        }
        let tc = group::get_first(d, &g).nil();
        // math.ceil(g:GetFirst():GetBaseAttack()/2)
        let atk = card::get_base_attack(d, tc);
        let dmg = (atk as f64 / 2.0).ceil() as i64;
        duel::damage(d, 1 - tp, dmg, REASON_EFFECT, false, None);
    }
}
