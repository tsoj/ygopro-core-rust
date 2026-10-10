//! Silent Magician LV8
use super::prelude::*;

pub const ID: u32 = 72443568;
pub const INFO: ScriptInfo = ScriptInfo { lv: Some((8, SET_SILENT_MAGICIAN)), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux_false());
    card::register_effect(d, c, e1, false);
    // immune spell
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellEffect() and te:GetOwnerPlayer()~=e:GetHandlerPlayer()
    let e = a.e(0);
    let te = a.e(1);
    effect::is_spell_effect(d, te) && effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
}
