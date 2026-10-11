//! Megalith Hagith
use super::prelude::*;

pub const ID: u32 = 90444325;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Add 1 "Megalith" Spell/Trap from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // local ritual_params={handler=c,lvtype=RITPROC_GREATER,forcedselection=function(e,tp,g,sc) return g:IsContains(e:GetHandler()) end}
    let ritual_params = ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        forcedselection: Some(std::sync::Arc::new(forcedselection)),
        ..Default::default()
    };
    // Ritual Summon 1 Ritual Monster from your hand, by Tributing monsters ...
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    let rtg = ritual::target(d, ritual_params.clone());
    effect::set_target(d, e2, Some(rtg));
    let rtop = ritual::operation(d, ritual_params);
    effect::set_operation(
        d,
        e2,
        Some(operation(move |d, a| {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // if c:IsRelateToEffect(e) and c:IsControler(tp) then Ritual.Operation(ritual_params)(e,tp,eg,ep,ev,re,r,rp) end
            if card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, a.tp) {
                rtop.call(d, &eff_values(a));
            }
        })),
    );
    card::register_effect(d, c, e2, false);
}
/// `e,tp,eg,ep,ev,re,r,rp` as the core passes them to an operation
fn eff_values(a: &EffArgs) -> Vec<Value> {
    vec![
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
fn forcedselection(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    let c = effect::get_handler(d, e).nil();
    (group::is_contains(g, c), false)
}
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MEGALITH as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
