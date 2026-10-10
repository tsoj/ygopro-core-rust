//! Malefic Cyber End Dragon
use super::prelude::*;

pub const ID: u32 = 1710476;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1546123], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:SetUniqueOnField(1,1,aux.MaleficUniqueFilter(c),LOCATION_MZONE)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Function(aux::malefic_unique_filter()), Some(LOCATION_MZONE));
    // aux.AddMaleficSummonProcedure(c,1546123,LOCATION_EXTRA)
    aux::add_malefic_summon_procedure(d, c, Some(1546123), LOCATION_EXTRA, None);
    // selfdes
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e7, EFFECT_SELF_DESTROY);
    // e7:SetCondition(s.descon)
    effect::set_condition(d, e7, Some(condition(descon)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // cannot announce
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e8, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e8:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e8, LOCATION_MZONE, 0);
    // e8:SetTarget(s.antarget)
    effect::set_target(d, e8, Some(effect_card_fn(antarget)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // spson
    // local e9=Effect.CreateEffect(c)
    let e9 = effect::create_effect(d, c);
    // e9:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e9, EFFECT_TYPE_SINGLE);
    // e9:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e9, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e9:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e9, EFFECT_SPSUMMON_CONDITION);
    // e9:SetValue(aux.FALSE)
    effect::set_value_fn(d, e9, aux_false());
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
}
/// `s.descon(e)`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.antarget(e,c)`
fn antarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    c != effect::get_handler(d, e).nil()
}
