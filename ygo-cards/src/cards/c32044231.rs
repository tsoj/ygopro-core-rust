//! Rank-Up-Magic Magical Force
//! ＲＵＭ－マジカル・フォース
use super::prelude::*;

pub const ID: u32 = 32044231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon 1 Rank 5 Spellcaster Xyz monster from your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matspfilter(c,e,tp)`
fn matspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let fc = CardSet::from_card(c);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&fc), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsType(TYPE_XYZ) and c:IsRace(RACE_SPELLCASTER)
    //     and c:IsRank(4) and Duel.IsExistingMatchingCard(s.xyzspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_rank(d, c, &[4])
        && duel::is_existing_matching_card(d, Some(&|d, x| xyzspfilter(d, x, e, tp, c, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.xyzspfilter(c,e,tp,mc,pg)`
fn xyzspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRank(5) and c:IsRace(RACE_SPELLCASTER) and c:IsType(TYPE_XYZ)
    //     and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[5])
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.matspfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && matspfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingTarget(s.matspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_target(d, Some(&|d, x| matspfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.matspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| matspfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let ftc = CardSet::from_card(tc);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&ftc), tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and not pg:IsContains(tc))
    //     or not Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then return end
    if pg.len() > 1 || (pg.len() == 1 && !group::is_contains(&pg, tc)) {
        return;
    }
    if !duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // tc:NegateEffects(c)
    card::negate_effects(d, tc, c, None, false, None);
    // if tc:IsImmuneToEffect(e) then return end
    if card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.xyzspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,pg):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| xyzspfilter(d, x, a.e, tp, tc, &pg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // sc:SetMaterial(tc)
    card::set_material(d, sc, CardSet::from_card(tc));
    // Duel.Overlay(sc,tc)
    duel::overlay(d, sc, tc, false);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SpecialSummonStep(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // if Duel.SpecialSummonComplete()==0 or not (c:IsRelateToEffect(e) and c:IsCanBeXyzMaterial(sc,tp,REASON_EFFECT)) then return end
        if duel::special_summon_complete(d) == 0 || !(card::is_relate_to_effect(d, c, a.e) && card::is_can_be_xyz_material(d, c, Some(sc), Some(tp), Some(REASON_EFFECT))) {
            return;
        }
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.Overlay(sc,c)
        duel::overlay(d, sc, c, false);
    }
}
