//! コンタクト / Contact
use super::prelude::*;

pub const ID: u32 = 16616620;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRYSALIS, SET_NEO_SPACIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and g:IsExists(Card.ListsCode,1,nil,c:GetCode())
    let codes = card::get_code(d, c);
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, g, &|d, x| card::lists_code(d, x, &codes), 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_CHRYSALIS)`
fn chrysalis_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CHRYSALIS as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_CHRYSALIS),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&chrysalis_faceup), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 and g:FilterCount(Card.IsAbleToGrave,nil)==#g and Duel.GetMZoneCount(tp,g)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,g) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && group::filter_count(d, &g, &|d, c| card::is_able_to_grave(d, c, None), NO_EX) == g.len()
            && duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp, &g)),
                a.tp,
                LOCATION_HAND | LOCATION_DECK,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_CHRYSALIS),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&chrysalis_faceup), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if Duel.SendtoGrave(g,REASON_EFFECT)==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::send_to_grave(d, g, REASON_EFFECT, None, None) == 0
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0
    {
        return;
    }
    // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let operated = duel::get_operated_group(d);
    let og = group::filter(d, &operated, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,og)
    let sg = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| spfilter(d, c, a.e, a.tp, &og)),
        a.tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
