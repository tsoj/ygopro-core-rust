//! Dragunity Knight - Romulus
use super::prelude::*;

pub const ID: u32 = 11969228;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], listed_names: &[62265044], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,s.matfilter,2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 2, Some(2), None, None, None);
    // Add 1 "Dragunity" spell/trap or 1 "Dragon Ravine" from deck
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
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon 1 dragon or winged beast monster from hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,sc,st,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_WINGEDBEAST,sc,st,tp) and not c:IsType(TYPE_TOKEN,sc,st,tp)
    card::is_race_ctx(d, c, RACE_DRAGON | RACE_WINGEDBEAST, sc, st, Some(tp)) && !card::is_type_ctx(d, c, TYPE_TOKEN, sc, st, Some(tp))
}
/// `s.thcon(e,tp,...)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_DRAGUNITY) and c:IsSpellTrap()) or c:IsCode(62265044) and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_spell_trap(d, c)) || (card::is_code(d, c, &[62265044]) && card::is_able_to_hand(d, c, None))
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
/// `s.seqcfilter(c,tp,lg)`
fn seqcfilter(d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsPreviousLocation(LOCATION_EXTRA) and lg:IsContains(c)
    card::is_race(d, c, RACE_DRAGON) && card::is_previous_location(d, c, LOCATION_EXTRA) && group::is_contains(lg, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // return eg:IsExists(s.seqcfilter,1,nil,tp,lg)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| seqcfilter(d, c, &lg), 1, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_WINGEDBEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_race(d, c, RACE_DRAGON | RACE_WINGEDBEAST) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // --Negate its effects
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
            // --Cannot be used as link material
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetDescription(3312)
            effect::set_description(d, e3, 3312);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
            effect::set_code(d, e3, EFFECT_CANNOT_BE_LINK_MATERIAL);
            // e3:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e3, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e3:SetValue(1)
            effect::set_value(d, e3, 1);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3,true)
            card::register_effect(d, tc, e3, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
