//! First of the Dragons
use super::prelude::*;

pub const ID: u32 = 10817524;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), st, Some(tp)))), 2)]);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux::fuslimit());
    card::register_effect(d, c, e1, false);
    // indes
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value_fn(d, e2, effect_card_value(indval));
    card::register_effect(d, c, e2, false);
    // immune
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e3, lua_fn(efilter));
    card::register_effect(d, c, e3, false);
}
fn indval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return not c:IsType(TYPE_NORMAL)
    (!card::is_type(d, c, TYPE_NORMAL)) as i64
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsMonsterEffect() and te:GetOwner()~=e:GetOwner()
    let te = a.e(1);
    effect::is_monster_effect(d, te) && effect::get_owner(d, te) != effect::get_owner(d, a.e(0))
}
