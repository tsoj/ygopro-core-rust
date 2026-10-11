//! Performapal U Go Golem
use super::prelude::*;

pub const ID: u32 = 73511233;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGICIAN, SET_ODD_EYES, SET_PERFORMAPAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // add card to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(thcon)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // register when Pendulum summoned
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_condition(d, e2, Some(condition(effcon)));
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
    // perform a fusion summon
    // local params = {nil,Fusion.CheckWithHandler(Fusion.OnFieldMat(aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON))),nil,nil,Fusion.ForcedHandler}
    let params = fusion::SummonEff {
        matfilter: Some(fusion::check_with_handler(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_race(d, c, RACE_DRAGON))))),
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    };
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e3, Some(tg));
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e3, Some(op));
    card::register_effect(d, c, e3, false);
}
/// `s.thcfilter(c,tp)`
fn thcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_fusion_summoned(d, c)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(s.thcfilter,1,nil,tp)
    match a.eg.clone() {
        Some(eg) => group::is_exists(d, &eg, &|d, c| thcfilter(d, c, a.tp), 1, NO_EX),
        None => false,
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && (card::is_set_card(d, c, &[SET_MAGICIAN as u16]) || card::is_set_card(d, c, &[SET_ODD_EYES as u16]) || card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]))
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let nv = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_pendulum_summoned(d, h)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
