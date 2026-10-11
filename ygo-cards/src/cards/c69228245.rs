//! Performapal Changeraffe
use super::prelude::*;

pub const ID: u32 = 69228245;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Enable pendulum summon
    pendulum::add_procedure(d, c, None, None);

    // Special summon back your monster that was destroyed by battle
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);

    // When normal summoned, negate 1 of opponent's monsters, also it cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);

    // Same as above, but when special summoned
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}

fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, tp)
}

fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    eg.len() == 1 && group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    if a.chk == Some(0) {
        return card::is_destructable(d, h, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && match tc {
                Some(tc) => card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None),
                None => false,
            };
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, tc.map(CardSet::from_card), 1, 0, 0);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    if card::is_relate_to_effect(d, c, e)
        && duel::destroy(d, c, REASON_EFFECT, None, None) != 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        if let Some(tc) = tc {
            if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None) != 0 {
                // Cannot be destroyed by battle
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, 3000);
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                effect::set_value(d, e1, 1);
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}

fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, _tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) {
        let tc = duel::first_target(d).nil();
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e) {
            card::set_card_target(d, c, tc);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_condition(d, e1, Some(condition(rcon)));
            card::register_effect(d, tc, e1, true);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
            card::register_effect(d, tc, e2, true);
        }
    }
}

fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
