//! Hidden Temples of Necrovalley
use super::prelude::*;

pub const ID: u32 = 70000776;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NECROVALLEY],
    listed_series: &[SET_GRAVEKEEPERS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // disable spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(lua_fn(sumlimit)));
    card::register_effect(d, c, e2, false);
    // self destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e3, Some(condition(sdcon)));
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GRAVEKEEPERS)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16])
}
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) and Duel.IsEnvironment(CARD_NECROVALLEY)
    duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSetCard(SET_GRAVEKEEPERS)
    let c = a.c(1);
    !card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16])
}
fn sdcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter1,0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     or not Duel.IsEnvironment(CARD_NECROVALLEY)
    !duel::is_existing_matching_card(d, Some(&cfilter1), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        || !duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
