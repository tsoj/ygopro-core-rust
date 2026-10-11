//! Megalith Phuloch
use super::prelude::*;

pub const ID: u32 = 8805651;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Add 1 "Megalith" card from your GY to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(e1cond)));
    effect::set_cost(d, e1, Some(cost::hard_once_per_chain(ID, None)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let ritual_params = ritual::RitualProc {
        filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_MEGALITH as u16]) && !card::is_code(d, c, &[ID]))),
        lvtype: RITPROC_GREATER,
        location: Some(LOCATION_GRAVE),
        forcedselection: Some(std::sync::Arc::new(forcedselection)),
        ..Default::default()
    };
    // Ritual Summon 1 "Megalith" Ritual Monster from your GY, by Tributing monsters from your hand or field, including this card on your field, whose total Levels equal or exceed the Level of the Ritual Monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_cost(d, e2, Some(cost::hard_once_per_chain(ID, None)));
    let rit_tg = ritual::target(d, ritual_params.clone());
    effect::set_target(d, e2, Some(rit_tg));
    let rit_op = ritual::operation(d, ritual_params);
    effect::set_operation(
        d,
        e2,
        Some(operation(move |d, a| {
            // if c:IsRelateToEffect(e) and c:IsControler(tp) then Ritual.Operation(ritual_params)(e,tp,eg,ep,ev,re,r,rp) end
            let c = effect::get_handler(d, a.e).nil();
            if card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, a.tp) {
                rit_op.call(d, &eff_values(a));
            }
        })),
    );
    card::register_effect(d, c, e2, false);
}
fn eff_values(a: &EffArgs) -> [Value; 8] {
    [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
fn e1cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
/// `forcedselection=function(e,tp,g,sc) return g:IsContains(e:GetHandler()) end`
fn forcedselection(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    let c = effect::get_handler(d, e).nil();
    (g.contains(c), false)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MEGALITH as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
