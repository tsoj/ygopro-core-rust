//! Gravekeeper's Heretic
use super::prelude::*;

pub const ID: u32 = 46955770;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immunity
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_NECROVALLEY)
    duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    let (e, te) = (a.e(0), a.e(1));
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
