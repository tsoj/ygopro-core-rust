//! Vulcarrion the Rotting Phoenix
use super::prelude::*;

pub const ID: u32 = 58354899;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(selfspcond)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Return this card from the field to the hand, then you can Special Summon 1 Zombie monster from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),tp,LOCATION_MZONE,0,1,nil) end`
fn selfspcond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::shuffle_hand(d, tp);
        let g = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            duel::break_effect(d);
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
