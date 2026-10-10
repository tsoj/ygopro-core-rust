//! Etude of the Branded
use super::prelude::*;

pub const ID: u32 = 45675980;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BYSTIAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Synchro Summon using a Dragon monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sctg)));
    effect::set_operation(d, e2, Some(operation(scop)));
    card::register_effect(d, c, e2, false);
    // Banished instead
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_TO_GRAVE_REDIRECT);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, 0, LOCATION_ALL);
    effect::set_condition(d, e3, Some(lua_fn(rmcon)));
    effect::set_target(d, e3, Some(effect_card_fn(rmtg)));
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e3, false);
}
/// `function(tp,sg,sc) return sg:IsExists(Card.IsRace,1,nil,RACE_DRAGON) end`
fn checkaddition(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_race(d, c, RACE_DRAGON), 1, NO_EX)
}
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        synchro::set_check_additional(d, Some(lua_fn(checkaddition)));
        let res = duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        synchro::set_check_additional(d, None);
        return res;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    synchro::set_check_additional(d, Some(lua_fn(checkaddition)));
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    } else {
        synchro::set_check_additional(d, None);
    }
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_BYSTIAL),...) end`
fn rmcon(d: &mut Duel, a: Args) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BYSTIAL as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(e,c)`: the cards of the opponent's that are sent to the GY by Synchro/Fusion/Link or a Ritual release
fn rmtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    card::get_owner(d, c) == 1 - tp
        && duel::is_player_can_remove(d, tp, Some((c, Some(REASON_EFFECT))))
        && card::get_reason_player(d, c) == 1 - tp
        && ((card::get_reason(d, c) & (REASON_RELEASE | REASON_RITUAL)) == (REASON_RELEASE | REASON_RITUAL)
            || card::is_reason(d, c, REASON_FUSION | REASON_SYNCHRO | REASON_LINK))
}
