//! The Dark Door
use super::prelude::*;

pub const ID: u32 = 30606547;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    card::register_effect(d, c, e2, false);
    // check
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(checkop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():GetFlagEffect(id)~=0 end`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `function(e,c) return c:GetFieldID()~=e:GetLabel() end`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    i64::from(card::get_field_id(d, c)) != effect::get_label(d, e)
}
/// `s.checkop`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetFlagEffect(id)~=0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return;
    }
    // local fid=eg:GetFirst():GetFieldID()
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    let fid = card::get_field_id(d, tc);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // e:GetLabelObject():SetLabel(fid)
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e2, &[fid as i64]);
}
