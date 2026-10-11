//! Winds Over the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 17197110;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon level 4 or lower "Ice Barrier" monsters from deck
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Add 1 of your "Ice Barrier" monsters, that is banished or in GY, to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(aux::exccon()));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_monster(d, c)
        && card::is_level_below(d, c, 4)
}
// function s.check(ct) ... return function(sg,tp) return aux.ChkfMMZ(#sg)(sg,nil,tp) and ct:GetClassCount(Card.GetCode)>=#sg end
fn check(d: &mut Duel, sg: &CardSet, tp: u8, ct: &CardSet) -> (bool, bool) {
    let n = sg.len();
    let (mmz, _) = aux::chkf_mmz(n as i32)(d, sg, None, tp, &CardSet::new(), None);
    let ok = mmz && group::get_class_count(d, ct, &|d, c| card::get_code(d, c)[0] as i64) >= n;
    (ok, false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let cg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    let ct = cg.len();
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 0 {
            return false;
        }
        return ct > 0
            && duel::check_release_group_cost(
                d,
                a.tp,
                Some(&|d, c| card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])),
                1,
                99,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet| check(d, sg, tp, &cg)),
                NO_EX,
            );
    }
    let mut maxc = ct;
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        maxc = 1;
    }
    let g = duel::select_release_group_cost(
        d,
        a.tp,
        Some(&|d, c| card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])),
        1,
        maxc,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet| check(d, sg, tp, &cg)),
        NO_EX,
    );
    duel::release(d, g.clone(), REASON_COST, None);
    effect::set_label(d, a.e, &[g.len() as i64]);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, g.len() as u8, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let ct = effect::get_label(d, a.e);
    if (ft as i64) < ct {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    if (g.len() as i64) < ct {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let n = ct as usize;
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(n),
        Some(n),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    if sg.len() > 0 {
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// Add 1 of your "Ice Barrier" monsters, that is banished or in GY
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED)
            && card::is_controler(d, chkc, a.tp)
            && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
