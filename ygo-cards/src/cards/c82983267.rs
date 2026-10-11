//! Purrelyeap!?
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 82983267;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PURRELY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Purrely" Xyz monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Shuffle 3 "Purrely" monsters to the deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn sptgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsFacedown() or not c:IsSetCard(SET_PURRELY) then return false end
    if card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_PURRELY as u16]) {
        return false;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg==0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && duel::is_existing_matching_card(d, Some(&|d, mc| spfilter(d, mc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_PURRELY)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and not c:IsRank(mc:GetRank())
    //     and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    if !card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp)) {
        return false;
    }
    if !(card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_PURRELY as u16])) {
        return false;
    }
    if duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    let rank = card::get_rank(d, mc);
    if card::is_rank(d, c, &[rank]) {
        return false;
    }
    card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && sptgfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| sptgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, c| sptgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsControler(tp) and tc:IsCanBeXyzMaterial() and not tc:IsImmuneToEffect(e)) then return end
    if !(card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, e)
        && card::is_controler(d, tc, tp)
        && card::is_can_be_xyz_material(d, tc, None, None, None)
        && !card::is_immune_to_effect(d, tc, e))
    {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and pg:GetFirst()~=tc) then return end
    if pg.len() > 1 || (pg.len() == 1 && group::get_first(d, &pg) != Some(tc)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    // sc:SetMaterial(tc)
    card::set_material(d, sc, CardSet::from_card(tc));
    // Duel.Overlay(sc,tc)
    duel::overlay(d, sc, CardSet::from_card(tc), false);
    if duel::special_summon_step(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) {
        let turn_ct = duel::get_turn_count(d, None);
        // Return it to the Extra Deck during the End Phase of the next turn
        aux::delayed_operation(
            d,
            sc,
            PHASE_END,
            ID,
            e,
            tp,
            Some(std::sync::Arc::new(return_oper)),
            Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_ct + 1)),
            None,
            Some(2),
            Some(aux::stringid(ID, 2)),
            None,
        );
    }
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    card::complete_procedure(d, sc);
}
/// `function(ag) Duel.SendtoDeck(ag,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end`
fn return_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_deck(d, ag, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    true
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_PURRELY) and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::send_to_deck(d, &tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
