//! Infinitrack Earth Slicer
use super::prelude::*;

pub const ID: u32 = 97584719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, None, None, false, None);
    // Attach an opponent's monster that this card destroyed by battle to this card as material
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(aux::bdocon()));
    effect::set_target(d, e1, Some(cost_target(atchtg)));
    effect::set_operation(d, e1, Some(operation(atchop)));
    card::register_effect(d, c, e1, false);
    // Destroy cards on the field equal to the number of materials you detach from this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    let detach_count: cost::Count = cost::Count::F(std::sync::Arc::new(|d: &mut Duel, e: EffectId, tp: u8| {
        duel::get_matching_group_count(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64
    }));
    let detach_op: cost::DetachOp = std::sync::Arc::new(|d: &mut Duel, e: EffectId, og: &CardSet| {
        effect::set_label(d, e, &[og.len() as i64]);
    });
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(detach_count), Some(detach_op))));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Special Summon this card from the GY in Defense Position
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ)
            && card::is_monster(d, bc.nil())
            && card::is_faceup(d, bc.nil())
            && card::is_can_be_xyz_material(d, bc.nil(), Some(c), Some(a.tp), Some(REASON_EFFECT));
    }
    duel::set_target_card(d, bc.nil());
    if card::is_location(d, bc.nil(), LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(bc.nil())), 1, a.tp, 0);
    }
    true
}
fn atchop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_relate_to_effect(d, bc.nil(), a.e)
        && card::is_monster(d, bc.nil())
        && card::is_faceup(d, bc.nil())
    {
        duel::overlay(d, c, bc.nil(), true);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    let ct = effect::get_label(d, a.e);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_MACHINE) && card::is_link_monster(d, c) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
