//! Malefic Red-Eyes Black Dragon
use super::prelude::*;

pub const ID: u32 = 55343236;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_REDEYES_B_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    let uf = aux::malefic_unique_filter(d, c);
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Function(uf), Some(LOCATION_MZONE));
    aux::add_malefic_summon_procedure(d, c, Some(CARD_REDEYES_B_DRAGON), LOCATION_DECK, None);
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
}
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
fn antarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) != effect::get_handler(d, e)
}
