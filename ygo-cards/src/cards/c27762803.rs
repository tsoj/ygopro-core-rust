//! Naturia Beetle
use super::prelude::*;

pub const ID: u32 = 27762803;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ad change
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_operation(d, e1, Some(operation(adop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_SWAP_BASE_AD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(con)));
    card::register_effect(d, c, e2, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
fn adop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        // if e:GetHandler():GetFlagEffect(id)==0 then
        if card::get_flag_effect(d, h, ID) == 0 {
            // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
            card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        } else {
            // e:GetHandler():ResetFlagEffect(id)
            card::reset_flag_effect(d, h, ID);
        }
    }
}
