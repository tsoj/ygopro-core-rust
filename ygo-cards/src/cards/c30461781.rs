//! Legacy of Yata-Garasu
use super::prelude::*;

pub const ID: u32 = 30461781;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card or 2 if the opponent controls a Spirit monster
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_SPIRIT) && card::is_faceup(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    let mut dr = 1;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 and ... and Duel.SelectOption(...)==1 then d=2 end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1
        && duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) == 1
    {
        dr = 2;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, dr);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, dr as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
