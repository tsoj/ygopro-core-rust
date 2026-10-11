//! Majestic Star Dragon
use super::prelude::*;

pub const ID: u32 = 7841112;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[21159309, CARD_STARDUST_DRAGON],
    listed_names: &[21159309, CARD_STARDUST_DRAGON], // Majestic Dragon
    synchro_nt_required: Some(1),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    synchro::add_majestic_procedure(
        d,
        c,
        mat_filter(|d, c, _sc, _st, _tp| card::is_code(d, c, &[21159309])),
        true,
        mat_filter(|d, c, _sc, _st, _tp| card::is_code(d, c, &[CARD_STARDUST_DRAGON])),
        true,
        synchro::non_tuner(None),
        false,
        vec![],
    );
    // Negate the activation of an opponent's card or effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Negate the effects of a monster the opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // activate limit
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_operation(d, e3, Some(operation(alop)));
    card::register_effect(d, c, e3, false);
    // Return itself to the Extra Deck and Special Summon 1 "Stardust Dragon"
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    // Multiple tuners
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e5, lua_fn(valcheck));
    card::register_effect(d, c, e5, false);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && a.ep == 1 - a.tp && duel::is_chain_negatable(d, a.ev as u8)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if duel::negate_activation(d, a.ev as u8) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
        let rh = effect::get_handler(d, re).nil();
        card::is_relate_to_effect(d, rh, re)
    } {
        duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
    }
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e2, false);
            duel::majestic_copy(d, c, tc, None);
        }
    }
}
fn alop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if effect::get_owner(d, re) == effect::get_owner(d, a.e) && !effect::is_has_property(d, re, EFFECT_FLAG_INITIAL, 0) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_target_range(d, e1, 1, 1);
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, h, e1, false);
    }
}
/// `function(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    effect::get_owner(d, re) == effect::get_owner(d, e) && !effect::is_has_property(d, re, EFFECT_FLAG_INITIAL, 0)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_STARDUST_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_able_to_extra(d, c)
        && duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, c, LOCATION_EXTRA)
        && tc.is_some_and(|tc| card::is_relate_to_effect(d, tc, a.e))
    {
        duel::special_summon(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_TUNER), 2, NO_EX) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
