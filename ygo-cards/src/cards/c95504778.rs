//! Triggering Wurm
use super::prelude::*;

pub const ID: u32 = 95504778;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is sent to the GY as Link Material for the Link Summon of a DARK monster: You can Special Summon this card from your GY ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card is destroyed or banished by an effect activated by a Link Monster: Draw 1 card
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_DRAW);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2a, EVENT_DESTROYED);
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    effect::set_condition(d, e2a, Some(condition(drcon)));
    effect::set_target(d, e2a, Some(cost_target(drtg)));
    effect::set_operation(d, e2a, Some(operation(drop_op)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_REMOVE);
    effect::set_condition(d, e2b, Some(condition(drcon_b)));
    card::register_effect(d, c, e2b, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE)
        && (a.r & REASON_LINK) == REASON_LINK
        && card::is_attribute(d, card::get_reason_card(d, c).nil(), ATTRIBUTE_DARK)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let link_monster = card::get_reason_card(d, c);
    if a.chk == Some(0) {
        let zones = card::get_linked_zone(d, link_monster.nil(), Some(tp)) & ZONES_MMZ;
        return zones > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), Some(tp), Some(zones));
    }
    let lm = link_monster.nil();
    effect::set_label_object(d, e, Value::Card(lm));
    card::create_effect_relation(d, lm, e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let link_monster = effect::get_label_object(d, e).as_card().nil();
    let zones = card::get_linked_zone(d, link_monster, Some(tp)) & ZONES_MMZ;
    if card::is_relate_to_effect(d, c, e)
        && card::is_relate_to_effect(d, link_monster, e)
        && zones > 0
        && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, Some(zones))
    {
        // It cannot be used as Link Material
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3312);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, true);
    }
    duel::special_summon_complete(d);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    card::is_reason(d, c, REASON_EFFECT)
        && effect::is_activated(d, re)
        && effect::is_monster_effect(d, re)
        && card::is_link_monster(d, effect::get_handler(d, re).nil())
}
fn drcon_b(d: &mut Duel, a: &EffArgs) -> bool {
    if !drcon(d, a) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    !card::is_reason(d, c, REASON_REDIRECT)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
