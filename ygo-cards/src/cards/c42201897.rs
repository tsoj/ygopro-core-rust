//! Azamina Determination
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 42201897;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AZAMINA, SET_SINFUL_SPOILS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send "Sinful Spoils" cards to the GY and Special Summon 1 "Azamina" Fusion Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Sinful Spoils" Spell/Trap from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sinfilter(c)`
fn sinfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SINFUL_SPOILS) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsSetCard(SET_AZAMINA) and c:IsType(TYPE_FUSION) and c:IsLevelAbove(4) and c:IsLevelBelow(lv) and not c:IsPublic()
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:CheckFusionMaterial()
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    card::is_set_card(d, c, &[SET_AZAMINA as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_level_above(d, c, 4)
        && card::is_level_below(d, c, lv)
        && !card::is_public(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::check_fusion_material(d, c, None, None, None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(s.sinfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&sinfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX) as u32;
        // return ct>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,ct*4+3)
        return ct > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ct * 4 + 3)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.sinfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(sinfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if #sg<=0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,#sg*4+3):GetFirst()
    let lv = sg.len() as u32 * 4 + 3;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, lv)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
    // local ct=tc:GetLevel()//4
    let ct = card::get_level(d, tc) / 4;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local ssg=sg:Select(tp,ct,ct,nil)
    let ssg = group::select(d, &sg, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // if #ssg==0 then return end
    if ssg.is_empty() {
        return;
    }
    // Duel.HintSelection(ssg)
    duel::hint_selection(d, ssg.clone(), true);
    // if Duel.SendtoDeck(ssg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and ssg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then
    if duel::send_to_deck(d, ssg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &ssg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX)
    {
        // tc:SetMaterial(nil)
        card::set_material(d, tc, CardSet::new());
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(tc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, CardSet::from_card(tc), SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SINFUL_SPOILS) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&setfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsSSetable() and Duel.SSet(tp,tc)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) && duel::s_set(d, a.tp, CardSet::from_card(tc), None, true) > 0 {
        // Cannot be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_CANNOT_ACT|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_CANNOT_ACT | RESET_PHASE | PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
