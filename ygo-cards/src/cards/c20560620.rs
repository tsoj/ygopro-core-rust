//! Emperor Tanuki's Critter Count
use super::prelude::*;

pub const ID: u32 = 20560620;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Tributed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can send 1 monster from your Extra Deck to the GY to Ritual Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_RITUAL_MATERIAL)
    effect::set_code(d, e3, EFFECT_EXTRA_RITUAL_MATERIAL);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTargetRange(LOCATION_EXTRA,0)
    effect::set_target_range(d, e3, LOCATION_EXTRA, 0);
    // e3:SetCondition(function(e) return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)==0 end)
    effect::set_condition(d, e3, Some(condition(e3_condition)));
    // e3:SetTarget(s.mttg)
    effect::set_target(d, e3, Some(effect_card_fn(mttg)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // e3:SetLabelObject({s.forced_replacement})
    effect::set_label_object(d, e3, Value::Multi(vec![Value::Fn(lua_fn(forced_replacement))]));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)==0 end`
fn e3_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) == 0
}
/// `s.mttg(e,c)`
fn mttg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local g=Duel.GetMatchingGroup(nil,e:GetHandlerPlayer(),LOCATION_EXTRA,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, None, p, LOCATION_EXTRA, 0, NO_EX);
    // return g:IsContains(c)
    group::is_contains(&g, c)
}
/// `s.forced_replacement(e,tp,sg,rc)`: returns `ct<=1, ct>1`
fn forced_replacement(d: &mut Duel, a: Args) -> Value {
    // local ct=sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
    let sg = a.g(2);
    let ct = group::filter_count(d, &sg, &|d, cc| card::is_location(d, cc, LOCATION_EXTRA), NO_EX);
    // return ct<=1,ct>1
    Value::Multi(vec![Value::Bool(ct <= 1), Value::Bool(ct > 1)])
}
