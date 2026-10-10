//! Chronicle Sorceress
use super::prelude::*;

pub const ID: u32 = 54878729;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON, CARD_DARK_MAGICIAN, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects, based on the Attributes in your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,code)`
fn tgfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    (card::is_code(d, c, &[code]) || card::lists_code(d, c, &[code])) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, CARD_BLUEEYES_W_DRAGON)), tp, LOCATION_DECK, 0, 1, NO_EX);
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, CARD_DARK_MAGICIAN)), tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let code = if effect::get_label(d, a.e) == 1 { CARD_BLUEEYES_W_DRAGON } else { CARD_DARK_MAGICIAN };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
