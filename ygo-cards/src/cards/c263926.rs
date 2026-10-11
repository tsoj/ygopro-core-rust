//! Bonze Alone
use super::prelude::*;

pub const ID: u32 = 263926;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be normal summoned if control a monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_condition(d, e1, Some(condition(sumcon_op)));
    card::register_effect(d, c, e1, false);
    // spsummon condition
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    effect::set_condition(d, e2, Some(condition(sumcon_op)));
    card::register_effect(d, c, e2, false);
    // Special summon from hand
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e3, EFFECT_FLAG_SPSUM_PARAM + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_target_range(d, e3, POS_FACEUP_ATTACK, 0);
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // Self-destruction if a monster is normal summoned to your field
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // Same as above, but if flip summoned
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e5, false);
    // Same as above, but if special summoned
    let e6 = effect::clone(d, e4);
    effect::set_code(d, e6, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e6, false);
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, e: EffectId) -> bool {
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) > 0
}
/// `s.sumcon` as a condition of a card effect (`function(e)`)
fn sumcon_op(d: &mut Duel, a: &EffArgs) -> bool {
    sumcon(d, a.e)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    match a.card(1) {
        // if c==nil then return true end
        None => true,
        Some(c) => {
            let p = card::get_controler(d, c);
            duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0 && !sumcon(d, e)
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, a.tp), 1, h.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0;
    }
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
