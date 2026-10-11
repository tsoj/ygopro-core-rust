//! Cynet Refresh
use super::prelude::*;

pub const ID: u32 = 36368606;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy all monsters in the Main Monster Zones
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Cyberse Link Monsters you control become unaffected
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(immcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_operation(d, e2, Some(operation(immop)));
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let ac = duel::get_attacker(d).nil();
    card::is_controler(d, ac, 1 - a.tp) && card::is_race(d, ac, RACE_CYBERSE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, LOCATION_MMZONE, LOCATION_MMZONE);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_MMZONE, LOCATION_MMZONE);
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        let og = duel::get_operated_group(d);
        for tc in group::iter(d, &og) {
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
        // Special Summon destroyed Cyberse Link Monsters during the End Phase
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_condition(d, e1, Some(condition(spcon)));
        effect::set_operation(d, e1, Some(operation(spop)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let owner = card::get_owner(d, c);
    card::is_race(d, c, RACE_CYBERSE)
        && card::is_link_monster(d, c)
        && card::has_flag_effect(d, c, ID, None)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(owner), None)
        && duel::get_location_count(d, owner, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    if g.is_empty() {
        return;
    }
    let turn_p = duel::get_turn_player(d);
    let order: [u8; 2] = if turn_p == 0 { [0, 1] } else { [1, 0] };
    for p in order {
        let mut tg = group::filter(d, &g, &|d, c| card::is_controler(d, c, p), NO_EX);
        let lc = duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0;
        if tg.len() as i32 > lc {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            tg = group::select(d, &tg, tp, lc as u16, lc as u16, false, NO_EX).nil();
        }
        for tc in group::iter(d, &tg) {
            duel::special_summon_step(d, tc, 0, tp, p, false, false, POS_FACEUP as u8, None);
        }
    }
    duel::special_summon_complete(d);
}
/// `s.immcon(e,tp,eg,ep,ev,re,r,rp)`
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.immfilter(c)`
fn immfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&immfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        // Unaffected by other cards' effects
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3100);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e1, lua_fn(immval));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
/// `function(e,re) return e:GetHandler()~=re:GetOwner() end`
fn immval(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0));
    let o = effect::get_owner(d, a.e(1));
    h != o
}
