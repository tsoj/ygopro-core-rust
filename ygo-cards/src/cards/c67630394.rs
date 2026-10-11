//! FA - Dark Knight Lancer
use super::prelude::*;

pub const ID: u32 = 67630394;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_XYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        3,
        Some(xyz::alter_filter(xyzfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Gains 300 ATK for each material attached to it and each card equipped to it
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(|d, args: Args| {
        let tc = args.c(1);
        ((card::get_overlay_count(d, tc) + card::get_equip_count(d, tc)) * 300) as i64
    }));
    card::register_effect(d, c, e1, false);
    // Add 1 "Xyz" card from your GY to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // Attach 1 monster your opponent controls to this card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_EQUIP);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atchcon)));
    effect::set_target(d, e3, Some(cost_target(atchtg)));
    effect::set_operation(d, e3, Some(operation(atchop)));
    card::register_effect(d, c, e3, false);
}
/// `s.xyzfilter(c,tp,xyzc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    card::is_faceup(d, c) && card::is_rank(d, c, &[5, 6]) && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_XYZ as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
fn atchconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let equip_target = card::get_equip_target(d, c).nil();
    card::is_controler(d, equip_target, tp)
}
fn atchcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| atchconfilter(d, c, a.tp), 1, NO_EX)
}
fn atchfilter(d: &mut Duel, c: CardId) -> bool {
    !card::is_type(d, c, TYPE_TOKEN) && card::is_able_to_change_controler(d, c)
}
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&atchfilter), a.tp, 0, LOCATION_MZONE, 1, c.into());
    }
    true
}
fn atchop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&atchfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if !card::is_immune_to_effect(d, tc, a.e) {
            duel::overlay(d, c, tc, true);
        }
    }
}
