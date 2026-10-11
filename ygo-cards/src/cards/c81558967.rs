//! Valkyrie Vierte
use super::prelude::*;

pub const ID: u32 = 81558967;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate cards from your Deck, up to the number of "Valkyrie" monsters you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "valkyrie" monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcond)
    effect::set_condition(d, e2, Some(condition(spcond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (used in the count below)
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE),tp,LOCATION_MZONE,0,e:GetHandler())
        let ct = duel::get_matching_group_count(d, Some(&faceup_valkyrie), a.tp, LOCATION_MZONE, 0, c.into());
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<ct then return false end
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < ct as i32 {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,ct)
        let g = duel::get_decktop_group(d, a.tp, ct as u32);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &|d, x| card::is_able_to_hand(d, x, None), NO_EX) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalSpell() or c:IsNormalTrap()
    card::is_normal_spell(d, c) || card::is_normal_trap(d, c)
}
// aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE)
fn faceup_valkyrie(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VALKYRIE as u16])
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE),tp,LOCATION_MZONE,0,e:GetHandler())
    let ct = duel::get_matching_group_count(d, Some(&faceup_valkyrie), a.tp, LOCATION_MZONE, 0, c.into()) as u32;
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, a.tp, ct);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let mut g = duel::get_decktop_group(d, a.tp, ct);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if g:IsExists(s.thfilter,1,nil) then
        if group::is_exists(d, &g, &thfilter, 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:FilterSelect(tp,s.thfilter,1,1,nil)
            let sg = group::filter_select(d, &g, a.tp, &thfilter, 1, 1, false, NO_EX).nil();
            // if sg:GetFirst():IsAbleToHand() then
            let first = group::get_first(d, &sg).nil();
            if card::is_able_to_hand(d, first, None) {
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - a.tp, sg.clone());
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, a.tp);
            } else {
                // Duel.SendtoGrave(sg,REASON_RULE)
                duel::send_to_grave(d, sg.clone(), REASON_RULE, None, None);
            }
            // g:Sub(sg)
            group::remove_card(&mut g, &sg);
            // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
            duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
        }
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
    }
}
// function s.spfilter1(c,e,tp)
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VALKYRIE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_VALKYRIE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spcond(e,tp,eg,ep,ev,re,r,rp)
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter1(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
