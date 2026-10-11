//! Ursarctic Drytron
use super::prelude::*;

pub const ID: u32 = 89771220;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[33250142, CARD_URSARCTIC_BIG_DIPPER, 58793369, 27693363, 97148796],
    listed_series: &[SET_URSARCTIC, SET_DRYTRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // If you would Tribute a monster(s) to activate an "Ursarctic" or "Drytron" monster's effect, you can banish this card from your GY instead
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_COST_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(lua_fn(ccond)));
    effect::set_value_fn(d, e2, lua_fn(repval));
    effect::set_operation(d, e2, Some(lua_fn(repop)));
    card::register_effect(d, c, e2, false);
}
// function(e) return e:GetHandler():IsAbleToRemoveAsCost() end
fn ccond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_able_to_remove_as_cost(d, c, None)
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_URSARCTIC_BIG_DIPPER, 58793369]) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    let ok = group::get_class_count(d, sg, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) == 2
        && group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 1
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (ok, false)
}
// function s.spfilter(c,e,tp,sg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: Option<&CardSet>) -> bool {
    let ex: Exclude = match sg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    card::is_code(d, c, &[33250142])
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// s.locations: the zones the target and activation search
fn locations(d: &mut Duel, tp: u8) -> u32 {
    if duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[27693363, 97148796]))),
        tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        NO_EX,
    ) {
        LOCATION_HAND | LOCATION_ONFIELD | LOCATION_DECK
    } else {
        LOCATION_HAND | LOCATION_ONFIELD
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let locs = locations(d, a.tp);
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, locs, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, a.tp, locs as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_DECK) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let locs = locations(d, a.tp);
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, locs, 0, NO_EX);
    if g.len() <= 1 {
        return;
    }
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    if rg.len() == 2 && duel::remove(d, &rg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        let Some(sc) = sc else { return };
        duel::break_effect(d);
        if duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
            card::complete_procedure(d, sc);
        }
    }
}
// function s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)
fn repval(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(2)).nil();
    if !(card::is_set_card(d, c, &[SET_URSARCTIC as u16, SET_DRYTRON as u16]) && card::is_monster(d, c)) {
        return false;
    }
    // extracon(base,e,tp,eg,ep,ev,re,r,rp)
    let mut full = vec![a.v(0).clone(), a.v(2).clone()];
    full.extend(a.0.iter().skip(3).cloned());
    match a.v(1) {
        Value::Fn(f) => f.call(d, &full).truthy(),
        _ => false,
    }
}
// function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST|REASON_REPLACE) end
fn repop(d: &mut Duel, a: Args) {
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_COST | REASON_REPLACE, None, None);
}
