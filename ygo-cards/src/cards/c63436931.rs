//! Crimson Dragon
use super::prelude::*;

pub const ID: u32 = 63436931;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CRIMSON_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Search 1 Spell/Trap that mentions "The Crimson Dragon"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Dragon Synchro Monster from your Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:ListsCode(CARD_CRIMSON_DRAGON) and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_CRIMSON_DRAGON]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.tgfilter(c,e,tp,mc)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(7) and c:IsType(TYPE_SYNCHRO) and not c:IsCode(CARD_CRIMSON_DRAGON)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetLevel(),mc)
    if !(card::is_faceup(d, c) && card::is_level_above(d, c, 7) && card::is_type(d, c, TYPE_SYNCHRO) && !card::is_code(d, c, &[CARD_CRIMSON_DRAGON])) {
        return false;
    }
    let lvl = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, lvl, Some(mc))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,lvl,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lvl: u32, mc: Option<CardId>) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO) and c:IsLevel(lvl) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    if !(card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_level(d, c, &[lvl])) {
        return false;
    }
    if duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc~=c and s.tgfilter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && chkc != c && tgfilter(d, chkc, a.e, a.tp, c);
    }
    // if chk==0 then return c:IsAbleToExtra() and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,e,tp,c) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c)
            && duel::is_existing_target(d, Some(&|d, cc| tgfilter(d, cc, a.e, a.tp, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,e,tp,c)
    duel::select_target(d, a.tp, Some(&|d, cc| tgfilter(d, cc, a.e, a.tp, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and c:IsLocation(LOCATION_EXTRA)) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_EXTRA))
    {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc:GetLevel()):GetFirst()
        let lvl = card::get_level(d, tc);
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, cc| spfilter(d, cc, a.e, a.tp, lvl, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sg) else { return };
        // sc:SetMaterial(nil)
        card::set_material(d, sc, CardSet::new());
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
