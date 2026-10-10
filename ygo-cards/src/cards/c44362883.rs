//! Branded Fusion
use super::prelude::*;

pub const ID: u32 = 44362883;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::lists_code_as_material(d, c, &[CARD_ALBAZ]))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            exactcount: Some(2),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    let fc = a.c(2);
    group::is_exists(
        d,
        &sg,
        &|d, c| card::is_summon_code(d, c, Some(fc), SUMMON_TYPE_FUSION as u64, tp, &[CARD_ALBAZ]),
        1,
        NO_EX,
    )
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_type(d, c, TYPE_FUSION)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except Fusion Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_type(d, c, TYPE_FUSION)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK) as i32);
    true
}
