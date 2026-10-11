//! Magikey Beast - Ansyalabolas
use super::prelude::*;

pub const ID: u32 = 45655875;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[99426088], listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summoned properly if to be revived by effect
    card::enable_revive_limit(d, c);
    // Fusion Proc
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_MAGIKEY as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
        ],
    );
    // Return fusion spell
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(thcon)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // change position
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_POSITION | CATEGORY_DEFCHANGE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
    // banish
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_BATTLE_DESTROY_REDIRECT);
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e3, false);
}
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), st, Some(tp)) && !card::is_type_ctx(d, c, TYPE_TOKEN, Some(fc), st, Some(tp))
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[99426088]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// check for attack pos monster
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16]))
}
fn posfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if card::is_faceup(d, c) && card::is_attack_pos(d, c) && card::is_can_change_position(d, c) {
        let mut att: u32 = 0;
        let grave = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_GRAVE, 0, NO_EX);
        for gc in group::iter(d, &grave) {
            att |= card::get_attribute(d, gc);
        }
        return card::get_attribute(d, c) & att != 0;
    }
    false
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && posfilter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| posfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| posfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_attack_pos(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
            if card::is_position(d, tc, POS_FACEUP_DEFENSE) {
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
                effect::set_value(d, e1, -1000);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
