//! Second Coin Toss
use super::prelude::*;

pub const ID: u32 = 36562627;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Redo your coin toss
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_TOSS_COIN_NEGATE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(coincon)));
    effect::set_operation(d, e1, Some(operation(coinop)));
    card::register_effect(d, c, e1, false);
}
fn coincon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and not Duel.HasFlagEffect(tp,id)
    a.ep == a.tp && !duel::has_flag_effect(d, a.tp, ID, None)
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, h, None) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.TossCoin(tp,ev)
        duel::toss_coin(d, a.tp, a.ev as u8);
    }
}
