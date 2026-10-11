//! The Agent of Force - Mars
use super::prelude::*;

pub const ID: u32 = 91123920;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune spell
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    card::register_effect(d, c, e1, false);
    // update atk,def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, _e: EffectId, te: EffectId) -> bool {
    // return te:IsSpellEffect()
    effect::is_spell_effect(d, te)
}
/// `s.envfilter(c)`
fn envfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_SANCTUARY_SKY)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SANCTUARY_SKY])
}
/// `s.val(e,c)`
fn val(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // if not (Duel.IsExistingMatchingCard(s.envfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    //     or Duel.IsEnvironment(CARD_SANCTUARY_SKY,tp)) then return 0 end
    let hp = effect::get_handler_player(d, e);
    if !(duel::is_existing_matching_card(d, Some(&envfilter), hp, LOCATION_ONFIELD, 0, 1, NO_EX)
        || duel::is_environment(d, CARD_SANCTUARY_SKY, Some(tp), None))
    {
        return 0;
    }
    // local v=Duel.GetLP(tp)-Duel.GetLP(1-tp)
    let v = duel::get_lp(d, tp) as i64 - duel::get_lp(d, 1 - tp) as i64;
    // if v>0 then return v else return 0 end
    if v > 0 { v } else { 0 }
}
