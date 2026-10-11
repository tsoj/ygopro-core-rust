//! Bettan Bat
use super::prelude::*;

pub const ID: u32 = 84079032;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the position of opponent's Normal or Special Summoned monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 3, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(postg)));
    effect::set_operation(d, e1, Some(operation(posop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Change position of a monster if it is flipped face-up or face-down
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e3, EVENT_CHANGE_POS);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(postg2)));
    effect::set_operation(d, e3, Some(operation(posop2)));
    card::register_effect(d, c, e3, false);
}
// function(e,tp,eg) return not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
fn posfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    (card::is_can_turn_set(d, c) || (!card::is_attack_pos(d, c) && card::is_can_change_position(d, c)))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_summon_player(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_MZONE)
}
/// `g:Select(tp,1,1,nil)` when several cards qualify, else `g:GetFirst()`
fn select_one(d: &mut Duel, tp: u8, g: &CardSet) -> CardSet {
    if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        group::select(d, g, tp, 1, 1, false, NO_EX).nil()
    } else {
        CardSet::from_card(group::get_first(d, g).nil())
    }
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && posfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| posfilter(d, c, a.e, a.tp), 1, NO_EX);
    }
    let g = group::filter(d, &eg, &|d, c| posfilter(d, c, a.e, a.tp), NO_EX);
    let tg = select_one(d, a.tp, &g);
    duel::set_target_card(d, &tg);
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(tg), 1, a.tp, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let coin = duel::toss_coin(d, a.tp, 1)[0];
    let pos = if coin == COIN_HEADS as i32 { POS_FACEUP_ATTACK } else { POS_FACEDOWN_DEFENSE };
    duel::change_position(d, tc, pos as u8, None, None, None, false);
}
fn posfilter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    (card::is_can_turn_set(d, c) || !card::is_position(d, c, POS_FACEUP_ATTACK))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_change_position(d, c)
        && ((card::is_position(d, c, POS_FACEUP) && card::is_previous_position(d, c, POS_FACEDOWN))
            || (card::is_position(d, c, POS_FACEDOWN_DEFENSE) && card::is_previous_position(d, c, POS_FACEUP)))
}
fn postg2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && card::is_location(d, chkc, LOCATION_MZONE) && posfilter2(d, chkc, a.e);
    }
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| posfilter2(d, c, a.e), 1, NO_EX);
    }
    let g = group::filter(d, &eg, &|d, c| posfilter2(d, c, a.e), NO_EX);
    let tg = select_one(d, a.tp, &g);
    duel::set_target_card(d, &tg);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(tg), 1, a.tp, 0);
    true
}
fn posop2(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let mut opt: u32 = 0;
    if card::is_position(d, tc, POS_FACEDOWN) || card::is_type(d, tc, TYPE_TOKEN) {
        opt = POS_FACEUP_ATTACK;
    } else if card::is_position(d, tc, POS_FACEUP_ATTACK) {
        opt = POS_FACEDOWN_DEFENSE;
    } else if card::is_position(d, tc, POS_FACEUP_DEFENSE) {
        opt = POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE;
    }
    if opt == 0 {
        return;
    }
    let pos = duel::select_position(d, a.tp, tc, opt as u8);
    if pos == 0 {
        return;
    }
    duel::change_position(d, tc, pos as u8, None, None, None, false);
}
