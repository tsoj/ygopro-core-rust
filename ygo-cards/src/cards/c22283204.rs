//! The Gaze of Timaeus
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 22283204;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL, 1784686], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster from your Extra Deck that mentions a targeted "Dark Magician" or "Dark Magician Girl" as material, by shuffling that target into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter(c,e,tp)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_DARK_MAGICIAN,CARD_DARK_MAGICIAN_GIRL) and c:IsFaceup()
    //     and c:IsCanBeFusionMaterial() and c:IsAbleToDeck()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL])
        && card::is_faceup(d, c)
        && card::is_can_be_fusion_material(d, c, None, None, None)
        && card::is_able_to_deck(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // if Duel.GetLocationCountFromEx(tp,tp,mc,c)<=0 then return false end
    if duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // local mustg=aux.GetMustBeMaterialGroup(tp,nil,tp,c,nil,REASON_FUSION)
    let mustg = aux::get_must_be_material_group_opt(d, tp, None, tp, Some(c), None, REASON_FUSION);
    // return c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(mc:GetCode()) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    //     and (#mustg==0 or (#mustg==1 and mustg:IsContains(mc)))
    let mc_codes = card::get_code(d, mc);
    card::is_type(d, c, TYPE_FUSION)
        && card::lists_code_as_material(d, c, &mc_codes)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION as u32, tp, false, false, None, None, None)
        && (mustg.is_empty() || (mustg.len() == 1 && group::is_contains(&mustg, mc)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc==0 then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc,e,tp) end
    // (never taken: a card or nil is never equal to 0 in Lua)
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| tdfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsCanBeFusionMaterial() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_fusion_material(d, tc, None, None, None) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, tc)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &sel) {
            // if tc:IsFacedown() then Duel.ConfirmCards(1-tp,tc) end
            if card::is_facedown(d, tc) {
                duel::confirm_cards(d, 1 - a.tp, tc);
            }
            // sc:SetMaterial(Group.FromCards(tc))
            card::set_material(d, sc, CardSet::from_card(tc));
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
            duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, sc, SUMMON_TYPE_FUSION as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
                // local turn_summoned=Duel.GetTurnCount()
                let turn_summoned = duel::get_turn_count(d, None);
                // --Banish it during the End Phase of the next turn
                // aux.DelayedOperation(sc,PHASE_END,id,e,tp,
                //     function(sc) Duel.Remove(sc,POS_FACEUP,REASON_EFFECT) end,
                //     function() return Duel.GetTurnCount()==turn_summoned+1 end,
                //     nil,2,aux.Stringid(id,1)
                // )
                aux::delayed_operation(
                    d,
                    sc,
                    PHASE_END,
                    ID,
                    a.e,
                    a.tp,
                    Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                        duel::remove(d, ag.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None);
                        true
                    })),
                    Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_summoned + 1)),
                    None,
                    Some(2),
                    Some(aux::stringid(ID, 1)),
                    None,
                );
            }
        }
    }
}
