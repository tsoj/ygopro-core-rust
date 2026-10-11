//! Mansion of the Dreadful Dolls
use super::prelude::*;

pub const ID: u32 = 36890111;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // "Gimmick Puppet" monsters you control cannot be destroyed by battle
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]))));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // "Gimmick Puppet" monsters you control are unaffected by your opponent's activated non-Xyz monster effects
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e3, lua_fn(immfilter));
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Gimmick Puppet" monster from your GY to your opponent's field in Defense Position
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}

fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}

fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}

fn immfilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    if !(effect::is_activated(d, te)
        && effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e)
        && effect::is_monster_effect(d, te))
    {
        return false;
    }
    let tc = effect::get_handler(d, te).nil();
    if !duel::is_chain_solving(d) || (card::is_relate_to_effect(d, tc, te) && card::is_faceup(d, tc)) {
        return !card::is_type(d, tc, TYPE_XYZ);
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_TYPE]);
    (info[0].int() & (TYPE_XYZ as i64)) == 0
}

fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, None);
    }
    duel::remove_overlay_card(d, a.tp, 1, 0, 1, 1, REASON_COST, None);
    true
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None)
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::get_first_target(d).first().copied().nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
