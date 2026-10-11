//! Mecha Phantom Beast Blackfalcon
use super::prelude::*;

pub const ID: u32 = 4417407;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_MECHA_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains the levels of all "Mecha Phantom Beast Token"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    effect::set_value_fn(d, e1, effect_card_value(lvval));
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle or effects while you control a Token
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Mecha Phantom Beast Token"
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    // Change target to face-up Defense Position
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_POSITION);
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_hint_timing(d, e5, TIMING_BATTLE_PHASE as u32, None);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_cost(d, e5, Some(cost_target(poscost)));
    effect::set_target(d, e5, Some(cost_target(postg)));
    effect::set_operation(d, e5, Some(operation(posop)));
    card::register_effect(d, c, e5, false);
}
/// `s.lvval(e,c)`
fn lvval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc) && card::is_code(d, cc, &[TOKEN_MECHA_PHANTOM_BEAST])), p, LOCATION_MZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, cc| card::get_level(d, cc) as i64)
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&tknfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            TOKEN_MECHA_PHANTOM_BEAST,
            duel::MonsterData {
                setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
                type_: Some(TYPES_TOKEN),
                attack: Some(0),
                defense: Some(0),
                level: Some(3),
                race: Some(RACE_MACHINE),
                attribute: Some(ATTRIBUTE_WIND),
            },
            None,
            None,
            0,
        )
    {
        let token = duel::create_token(d, tp, TOKEN_MECHA_PHANTOM_BEAST).nil();
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.poscfilter(c)`
fn poscfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_TOKEN)
}
/// `s.filter(c,e)`
fn filter_e(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    !card::is_position(d, c, POS_FACEUP_DEFENSE as u32) && card::is_can_change_position(d, c) && e.map_or(true, |e| card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.poscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let dg = duel::get_matching_group(d, Some(&|d, cc| filter_e(d, cc, Some(e))), a.tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&poscfilter), 1, 1, false, Some(&|d, sg: &CardSet, tp, exg| {
            aux::release_check_target(d, sg, tp, exg, &dg)
        }), NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&poscfilter), 1, 1, false, Some(&|d, sg: &CardSet, tp, exg| {
        aux::release_check_target(d, sg, tp, exg, &dg)
    }), NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter_e(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| filter_e(d, cc, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    duel::select_target(d, a.tp, Some(&|d, cc| filter_e(d, cc, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_position(d, tc, POS_FACEUP_DEFENSE as u32) {
        duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
