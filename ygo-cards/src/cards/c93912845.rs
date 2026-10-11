//! Revival Gift
use super::prelude::*;

pub const ID: u32 = 93912845;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[93912845], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1500),
        defense: Some(1500),
        level: Some(3),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_TUNER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(1 - tp), 0);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 3, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        let tc = duel::first_target(d);
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_DISABLE);
                effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, true);
                let e2 = effect::create_effect(d, h);
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
                card::register_effect(d, tc, e2, true);
            }
        }
    }
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(1 - tp), 0)
    {
        for _i in 1..=2 {
            let token = duel::create_token(d, tp, ID + 1).nil();
            duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
    duel::special_summon_complete(d);
}
