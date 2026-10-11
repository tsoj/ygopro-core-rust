//! Chronograph Sorcerer
use super::prelude::*;

pub const ID: u32 = 12289247;
/// `"Timegazer Magician"`
const TIMEGAZER_MAGICIAN: u32 = 20409757;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PENDULUM_DRAGON, SET_XYZ_DRAGON, SET_SYNCHRO_DRAGON, SET_FUSION_DRAGON],
    listed_names: &[CARD_ZARC, TIMEGAZER_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Take 1 "Timegazer Magician" and either place it in the Pendulum zone or Special Summon it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rptg)));
    effect::set_operation(d, e1, Some(operation(rpop)));
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand then you can Special Summon 1 monster from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 3));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Supreme King Z-Arc" from your Extra Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 5));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost_target(hncost)));
    effect::set_target(d, e3, Some(cost_target(hntg)));
    effect::set_operation(d, e3, Some(operation(hnop)));
    card::register_effect(d, c, e3, false);
}
/// `s.rpfilter(c,e,tp)`
fn rpfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[TIMEGAZER_MAGICIAN])
        && (!card::is_forbidden(d, c)
            || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn rptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rpfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn rpop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 6));
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| rpfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g).nil();
        let op = if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None)
        {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
        } else {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
        };
        if op == 0 {
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        } else {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d: &mut Duel, c: CardId| spcfilter(d, c, tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    let tp = a.tp;
    let e = a.e;
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_can_be_special_summoned(d, x, e, 0, tp, false, false, None, None, None)),
        tp,
        LOCATION_HAND,
        0,
        NO_EX,
    );
    if !g.is_empty()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
    {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(
        d,
        c,
        &[SET_PENDULUM_DRAGON as u16, SET_XYZ_DRAGON as u16, SET_SYNCHRO_DRAGON as u16, SET_FUSION_DRAGON as u16],
    ) && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (!card::is_location(d, c, LOCATION_MZONE) || card::is_faceup(d, c))
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, true))
}
/// `aux.PropertyTableFilter(Card.GetSetCard, ...)`: the setcodes of a card
fn setcard_values(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_set_card(d, c).into_iter().map(|x| x as i64).collect()
}
const DRAGON_SETS: &[i64] = &[SET_PENDULUM_DRAGON as i64, SET_XYZ_DRAGON as i64, SET_SYNCHRO_DRAGON as i64, SET_FUSION_DRAGON as i64];
/// `s.rescon(checkfunc)`: `function(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let checkfunc = aux::property_table_filter(setcard_values, DRAGON_SETS);
    if !group::check_different_property(d, sg, &checkfunc) {
        return (false, true);
    }
    let e = e.nil();
    let h = effect::get_handler(d, e).nil();
    let mut sg_h = sg.clone();
    group::add_card(&mut sg_h, &CardSet::from_card(h));
    let ex: Exclude = (&sg_h).into();
    let res = duel::is_existing_matching_card(d, Some(&|d, x| hnfilter(d, x, e, tp, ex)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (res, false)
}
/// `s.hnfilter(c,e,tp,sg)`
fn hnfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ex: Exclude) -> bool {
    card::is_code(d, c, &[CARD_ZARC])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
}
fn hncost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let mg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    let rc: aux::Rescon = &rescon;
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && matches!(
                aux::select_unselect_group(d, &mg, Some(a.e), a.tp, Some(4), Some(4), Some(rc), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    let sg = aux::select_unselect_group(d, &mg, Some(a.e), a.tp, Some(4), Some(4), Some(rc), 1, a.tp, Some(HINTMSG_REMOVE as u64), Some(rc), None, false)
        .unwrap_or_default();
    let mut sg = sg;
    group::add_card(&mut sg, &CardSet::from_card(c));
    duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn hntg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn hnop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| hnfilter(d, c, e, tp, NO_EX)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
