//! Super Quantum Black Layer
use super::prelude::*;

pub const ID: u32 = 92034192;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPER_QUANT, SET_SUPER_QUANTAL_MECH_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard 1 other card, and if you do, Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Super Quantal Mech Beast" Xyz Monster from your Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(xyzsptg)));
    effect::set_operation(d, e2, Some(operation(xyzspop)));
    card::register_effect(d, c, e2, false);
    // Add 1 "Super Quant" Spell from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsMonsterEffect() end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, cc| card::is_discardable(d, cc, Some(REASON_EFFECT), None)),
            tp,
            LOCATION_HAND,
            0,
            1,
            c.into(),
        ) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    let n = duel::discard_hand(
        d,
        tp,
        Some(&|d, cc| card::is_discardable(d, cc, Some(REASON_COST), None)),
        1,
        1,
        REASON_EFFECT | REASON_DISCARD,
        exc,
    );
    if n > 0 && card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.xyzspfilter(c,e,tp,mc,chk_att)`
fn xyzspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, chk_att: bool) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANTAL_MECH_BEAST as u16])
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && (!chk_att
            || duel::is_existing_matching_card(
                d,
                Some(&|d, cc| attachfilter(d, cc, tp, c)),
                tp,
                LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            ))
}
/// `s.attachfilter(c,tp,xc)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8, xc: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && card::is_monster(d, c)
        && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.xyzsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyzsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(
                d,
                Some(&|d, cc| xyzspfilter(d, cc, a.e, tp, c, true)),
                tp,
                LOCATION_EXTRA,
                0,
                1,
                NO_EX,
            );
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.xyzspop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c)
        || !card::is_relate_to_effect(d, c, a.e)
        || card::is_controler(d, c, 1 - tp)
        || card::is_immune_to_effect(d, c, a.e)
    {
        return;
    }
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    if pg.len() > 1 || (pg.len() == 1 && !group::is_contains(&pg, c)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| xyzspfilter(d, cc, a.e, tp, c, false)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let xc = match group::get_first(d, &sel) {
        Some(xc) => xc,
        None => return,
    };
    card::set_material(d, xc, CardSet::from_card(c));
    duel::overlay(d, xc, CardSet::from_card(c), false);
    if duel::special_summon(d, CardSet::from_card(xc), SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    card::complete_procedure(d, xc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    let sel2 = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| attachfilter(d, cc, tp, xc)),
        tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if let Some(ac) = group::get_first(d, &sel2) {
        duel::overlay(d, xc, CardSet::from_card(ac), false);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
