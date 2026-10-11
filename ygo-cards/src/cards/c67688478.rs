//! Gemini Imps
use super::prelude::*;

pub const ID: u32 = 67688478;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE | CATEGORY_DESTROY | CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.ep == tp
        || (!effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && !effect::is_monster_effect(d, a.re.nil()))
        || !duel::is_chain_negatable(d, a.ev as u8)
    {
        return false;
    }
    let operation_info = duel::get_operation_info(d, a.ev as u32, CATEGORY_HANDES);
    let poperation_info = duel::get_possible_operation_info(d, a.ev as u32, CATEGORY_HANDES);
    let ok = |p: u8| p == 1 - tp || p as u32 == PLAYER_EITHER as u32 || p as u32 == PLAYER_ALL as u32;
    effect::is_has_category(d, a.re.nil(), CATEGORY_HANDES)
        && ((operation_info.is_some_and(|op| ok(op.op_player)) || poperation_info.is_some_and(|op| ok(op.op_player))))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) && duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) != 0 {
            duel::break_effect(d);
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
