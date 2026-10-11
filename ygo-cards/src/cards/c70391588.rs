//! Star Siphon
use super::prelude::*;

pub const ID: u32 = 70391588;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[70391589],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// The token's data with the given level: `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,lv,RACE_SPELLCASTER,ATTRIBUTE_DARK)`.
fn token_data(lv: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(lv),
        race: Some(RACE_SPELLCASTER),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_can_be_effect_target(d, c, Some(e))) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(lv), None, None, 0)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&a.eg.clone().nil(), chkc) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, a.e, a.tp), 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, a.tp, &|d, c| filter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, &g);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d);
    let mut lv: u32 = 1;
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            lv = card::get_level(d, tc);
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
    if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(lv), None, None, 0) {
        return;
    }
    let token = duel::create_token(d, a.tp, ID + 1).nil();
    duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    let h = effect::get_handler(d, a.e).nil();
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
    effect::set_value(d, e2, lv as i64);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, token, e2, false);
    duel::special_summon_complete(d);
}
