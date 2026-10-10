//! Meliae of the Trees
use super::prelude::*;

pub const ID: u32 = 64245689;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_EARTH),3,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_EARTH, sc, st, Some(tp)))), Some(3), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_PLANT) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_race(d, c, RACE_PLANT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetLabel()==2 and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 2 && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // local b1=Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Send 1 Plant monster from your Deck to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Special Summon 1 Plant monster from your GY in Defense Position
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
