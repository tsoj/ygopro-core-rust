//! Altergeist Kunquery
use super::prelude::*;

pub const ID: u32 = 52927340;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Negate effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}

fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    let faceup_altergeist = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ALTERGEIST as u16]));
    card::is_controler(d, at, 1 - a.tp)
        && duel::is_existing_matching_card(d, Some(&faceup_altergeist), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}

fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        duel::negate_attack(d);
    }
}

fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}

fn rcon(d: &mut Duel, a: Args) -> bool {
    let owner = effect::get_owner(d, a.e(0)).nil();
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_has_card_target(d, owner, h)
}

fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::get_first_target(d).first().copied();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    let Some(tc) = tc else { return };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e)) {
        return;
    }
    card::set_card_target(d, c, tc);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_condition(d, e1, Some(lua_fn(rcon)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, tc, e1, false);
    if card::is_type(d, tc, TYPE_TRAPMONSTER) {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e2, EFFECT_DISABLE_TRAPMONSTER);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
    }
}
