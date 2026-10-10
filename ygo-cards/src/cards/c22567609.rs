//! Nimble Momonga
use super::prelude::*;

pub const ID: u32 = 22567609;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER | CATEGORY_SPECIAL_SUMMON | CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && (card::get_reason(d, c) & REASON_BATTLE) != 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (tp, e) = (a.tp, a.e);
    if duel::recover(d, tp, 1000, REASON_EFFECT, false, None) > 0 {
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        if ft <= 0 {
            return;
        }
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = group::select(d, &g, tp, 1, ft as u16, false, NO_EX).nil();
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        }
    }
}
