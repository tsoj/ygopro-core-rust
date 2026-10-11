//! Sin Stardust Dragon
use super::prelude::*;

pub const ID: u32 = 36521459;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // c:SetUniqueOnField(1,1,aux.MaleficUniqueFilter(c),LOCATION_MZONE)
    let uf = aux::malefic_unique_filter(d, c);
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Function(uf), Some(LOCATION_MZONE));
    // aux.AddMaleficSummonProcedure(c,CARD_STARDUST_DRAGON,LOCATION_EXTRA)
    aux::add_malefic_summon_procedure(d, c, Some(CARD_STARDUST_DRAGON), LOCATION_EXTRA, None);
    // selfdes
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_code(d, e7, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e7, Some(condition(descon)));
    card::register_effect(d, c, e7, false);
    // cannot announce
    let e8 = effect::create_effect(d, c);
    effect::set_type(d, e8, EFFECT_TYPE_FIELD);
    effect::set_range(d, e8, LOCATION_MZONE);
    effect::set_code(d, e8, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_target_range(d, e8, LOCATION_MZONE, 0);
    effect::set_target(d, e8, Some(effect_card_fn(antarget)));
    card::register_effect(d, c, e8, false);
    // indes
    let e9 = effect::create_effect(d, c);
    effect::set_type(d, e9, EFFECT_TYPE_FIELD);
    effect::set_code(d, e9, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e9, LOCATION_MZONE);
    effect::set_target_range(d, e9, LOCATION_FZONE, LOCATION_FZONE);
    effect::set_value(d, e9, 1);
    card::register_effect(d, c, e9, false);
    // spson
    let ea = effect::create_effect(d, c);
    effect::set_type(d, ea, EFFECT_TYPE_SINGLE);
    effect::set_property(d, ea, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, ea, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, ea, aux_false());
    card::register_effect(d, c, ea, false);
}
/// `s.descon(e)`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.antarget(e,c)`
fn antarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c != h
}
