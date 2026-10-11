//! Gagaga Magician - Gagaga Magic
use super::prelude::*;

pub const ID: u32 = 12908094;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_ZUBABA, SET_GAGAGA, SET_GOGOGO, SET_DODODO, SET_DJINN, SET_HEROIC_CHAMPION, SET_NUMBER, SET_ZW, SET_ZS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can Special Summon both it and 1 other "Zubaba", "Gagaga", ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `local function samexyzlevel(handler,mc,lv)`: an XYZ_LEVEL effect on `mc` that is returned
fn samexyzlevel(d: &mut Duel, handler: CardId, mc: CardId, lv: u32) -> EffectId {
    // When you do, treat the Level of 1 of the monsters the same as the other monster's
    let e1 = effect::create_effect(d, handler);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
    effect::set_value(d, e1, lv as i64);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, mc, e1, true);
    e1
}
/// `s.handgyspfilter(c,e,tp,mc)`
fn handgyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    if card::is_code(d, c, &[ID]) {
        return false;
    }
    if !(card::is_set_card(d, c, &[SET_ZUBABA as u16, SET_GAGAGA as u16, SET_GOGOGO as u16, SET_DODODO as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    let lv_mc = card::get_level(d, mc);
    let e1 = samexyzlevel(d, c, c, lv_mc);
    let lv_c = card::get_level(d, c);
    let e2 = samexyzlevel(d, c, mc, lv_c);
    let mut mg = CardSet::from_card(mc);
    group::add_card(&mut mg, &CardSet::from_card(c));
    let res = duel::is_existing_matching_card(d, Some(&|d, sc| extraspfilter(d, sc, e, tp, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    effect::reset(d, e1);
    effect::reset(d, e2);
    res
}
/// `s.extraspfilter(c,e,tp,mg)`
fn extraspfilter(d: &mut Duel, c: CardId, _e: EffectId, _tp: u8, mg: &CardSet) -> bool {
    card::is_set_card(
        d,
        c,
        &[SET_ZUBABA as u16, SET_GAGAGA as u16, SET_GOGOGO as u16, SET_DODODO as u16, SET_DJINN as u16, SET_HEROIC_CHAMPION as u16, SET_NUMBER as u16, SET_ZW as u16, SET_ZS as u16],
    ) && card::is_xyz_monster(d, c)
        && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, a.tp, NO_EX, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, sc| handgyspfilter(d, sc, a.e, a.tp, c)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    if !(card::is_relate_to_effect(d, c, e) && duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 >= 2) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(move |d: &mut Duel, x: CardId| handgyspfilter(d, x, e, tp, c));
    let sel = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else { return };
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(sc));
    if duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 2 {
        let lv_sc = card::get_level(d, sc);
        let e1 = samexyzlevel(d, c, c, lv_sc);
        let lv_c2 = card::get_level(d, c);
        let e2 = samexyzlevel(d, c, sc, lv_c2);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let mg = g.clone();
        let xyz_sel = duel::select_matching_card(d, tp, Some(&|d, x| extraspfilter(d, x, e, tp, &mg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(xyz) = group::get_first(d, &xyz_sel) {
            duel::xyz_summon(d, tp, xyz, Some(g), None, 2, 2);
        } else {
            effect::reset(d, e1);
            effect::reset(d, e2);
        }
    }
}
