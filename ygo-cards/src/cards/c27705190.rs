//! World Legacy's Mind Meld
use super::prelude::*;

pub const ID: u32 = 27705190;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KRAWLER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make an opponent's monster effect become "return 1 face-up monster your opponent controls to the hand"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(chngtg)));
    effect::set_operation(d, e1, Some(operation(chngop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Krawler" monster from your hand, Deck, or GY to your zone a Link Monster points to
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_KRAWLER as u16]))),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
// function s.chngtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn chngtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))),
            a.rp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    true
}
// function s.chngop(e,tp,eg,ep,ev,re,r,rp)
fn chngop(d: &mut Duel, a: &EffArgs) {
    let g = CardSet::new();
    duel::change_target_card(d, a.ev as u8, &g);
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::hint_selection(d, &g, true);
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
// function s.tgfilter(c,e,tp)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local zone=c:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && zone > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, e, tp, zone)),
            tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
}
// function s.spfilter(c,e,tp,zone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    card::is_set_card(d, c, &[SET_KRAWLER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(tp), Some(zone))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let zone = card::get_linked_zone(d, tc, Some(a.tp)) & ZONES_MMZ;
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, zone))),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        if !g.is_empty() && duel::special_summon(d, &g, 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, Some(zone)) > 0 {
            duel::confirm_cards(d, 1 - a.tp, &g);
        }
    }
}
