//! Evil HERO Adusted Gold
use super::prelude::*;

pub const ID: u32 = 13650422;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_condition(d, e2, Some(lua_fn(atkcon)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
        && (card::is_code(d, c, &[CARD_DARK_FUSION]) || card::lists_code(d, c, &[CARD_DARK_FUSION]))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `function(e) return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_FUSION),...) end`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
