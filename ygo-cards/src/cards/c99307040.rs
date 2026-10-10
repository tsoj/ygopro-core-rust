//! Horus the Black Flame Deity
use super::prelude::*;

pub const ID: u32 = 99307040;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_KING_SARCOPHAGUS], listed_series: &[SET_HORUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 card from the field to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetCost(s.tgcost)
    effect::set_cost(d, e1, Some(cost_target(tgcost)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
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
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_HORUS),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_KING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_HORUS as u16])),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    ) && duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_KING_SARCOPHAGUS])),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let tg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local exc=(#tg==1 and tg or nil)
    let exc = if tg.len() == 1 { (&tg).into() } else { NO_EX };
    // if chk==0 then return #tg>0 and not c:IsPublic()
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,exc) end
    if a.chk == Some(0) {
        return !tg.is_empty()
            && !card::is_public(d, c)
            && duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, exc);
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,exc)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, exc).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local code1,code2=c:GetPreviousCodeOnField()
    let codes = card::get_previous_code_on_field(d, c);
    let code1 = codes.first().copied();
    let code2 = codes.get(1).copied();
    // return c:IsPreviousControler(tp) and c:IsPreviousPosition(POS_FACEUP)
    //   and c:GetReasonPlayer()==1-tp and c:IsReason(REASON_EFFECT)
    //   and (code1==CARD_KING_SARCOPHAGUS or code2==CARD_KING_SARCOPHAGUS
    //   or (c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousSetCard(SET_HORUS) and code1~=id and code2~=id))
    card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::get_reason_player(d, c) == 1 - tp
        && card::is_reason(d, c, REASON_EFFECT)
        && (code1 == Some(CARD_KING_SARCOPHAGUS)
            || code2 == Some(CARD_KING_SARCOPHAGUS)
            || (card::is_previous_location(d, c, LOCATION_MZONE)
                && card::is_previous_set_card(d, c, &[SET_HORUS as u16])
                && code1 != Some(ID)
                && code2 != Some(ID)))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
