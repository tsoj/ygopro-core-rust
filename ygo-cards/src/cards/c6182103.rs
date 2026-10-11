//! Myutant Ultimus
use super::prelude::*;

pub const ID: u32 = 6182103;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MYUTANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 Level 8 or higher "Myutant" monsters
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))), 3)]);
    // Negate the activation of card/effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_cost(d, e1, Some(cost_target(negcost)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
    // Add up to 3 of your banished "Myutant" cards
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_level_above(d, c, 8) && card::is_set_card_ctx(d, c, &[SET_MYUTANT as u16], Some(fc), sumtype, Some(tp))
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
fn cfilter(d: &mut Duel, c: CardId, rtype: u32) -> bool {
    card::is_set_card(d, c, &[SET_MYUTANT as u16])
        && (!card::is_on_field(d, c) || card::is_faceup(d, c))
        && card::is_type(d, c, rtype)
        && card::is_able_to_remove_as_cost(d, c, None)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    let rtype = effect::get_active_type(d, re) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, rtype)), tp, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let rg = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, rtype)), tp, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let relation = card::is_relate_to_effect(d, rc, re);
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None));
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        let controler = card::get_controler(d, rc);
        let location = card::get_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, controler, location as i32);
    } else {
        let prev = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, prev as i32);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp && card::is_fusion_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, a.tp)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MYUTANT as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.ctcheck(sg,e,tp)`
fn ctcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::get_class_count(d, sg, &|d, c| card::get_main_card_type(d, c) as i64) == sg.len(), false)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(3), Some(&ctcheck), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    if !sg.is_empty() {
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
