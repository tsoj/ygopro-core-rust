//! Impcantation Inception
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 86758915;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IMPCANTATION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater({handler=c,matfilter=s.matfil})
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| matfil(d, c)))),
            ..Default::default()
        },
    );
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfil(c)`
fn matfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_IMPCANTATION)
    card::is_set_card(d, c, &[SET_IMPCANTATION as u16])
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_IMPCANTATION) and not c:IsCode(id) and c:IsAbleToGraveAsCost()
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_IMPCANTATION as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spcfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_IMPCANTATION) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_IMPCANTATION as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) and c:IsRelateToEffect(e) then
    if !g.is_empty() {
        // The number Duel.SpecialSummon returns is 0 or more, which Lua reads as true
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(c,nil,REASON_EFFECT)
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        }
    }
}
