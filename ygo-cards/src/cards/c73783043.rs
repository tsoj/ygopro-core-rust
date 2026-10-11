//! R-Genex Accelerator
use super::prelude::*;

pub const ID: u32 = 73783043;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsControler(tp) and not c:IsPublic()
    //     and c:IsPreviousLocation(LOCATION_DECK) and c:IsPreviousControler(tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GENEX as u16])
        && card::is_controler(d, c, tp)
        && !card::is_public(d, c)
        && card::is_previous_location(d, c, LOCATION_DECK)
        && card::is_previous_controler(d, c, tp)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and eg:IsExists(s.filter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d, c| filter(d, c, a.e, tp), 1, NO_EX);
    }
    // local g=eg:Filter(s.filter,nil,e,tp)
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.e, tp), NO_EX);
    // if #g==1 then
    if g.len() == 1 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
