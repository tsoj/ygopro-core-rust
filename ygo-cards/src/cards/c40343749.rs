//! House Duston
use super::prelude::*;

pub const ID: u32 = 40343749;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DUSTON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, rp) = (a.tp, a.rp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_BATTLE) then
    if card::is_reason(d, c, REASON_BATTLE) {
        // return c:GetReasonPlayer()~=tp and c:GetBattlePosition()&POS_FACEUP~=0
        return card::get_reason_player(d, c) != tp && (card::get_battle_position(d, c) & POS_FACEUP as u32) != 0;
    }
    // return rp~=tp and c:IsReason(REASON_DESTROY) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    rp != tp
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, tp)
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DUSTON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK|LOCATION_HAND,0,1,c,e,tp)
    card::is_set_card(d, c, &[SET_DUSTON as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| filter2(d, x, e, tp)),
            tp,
            LOCATION_DECK | LOCATION_HAND,
            0,
            1,
            c.into(),
        )
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DUSTON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    card::is_set_card(d, c, &[SET_DUSTON as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| filter1(d, c, e, tp)),
                tp,
                LOCATION_DECK | LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let mut g2 = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)<=0
    //     or #g1<=0 or #g2<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 <= 0
        || g1.len() <= 0
        || g2.len() <= 0
    {
        return;
    }
    // repeat ... until not (...)
    loop {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
        // sg1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let sg1 = duel::select_matching_card(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // tc=sg1:GetFirst()
        let tc = group::get_first(d, &sg1).nil();
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // sg2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let sg2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // tc=sg2:GetFirst()
        let tc = group::get_first(d, &sg2).nil();
        // Duel.SpecialSummon(tc,0,tp,1-tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        // g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
        g1 = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
        g2 = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // until not (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
        //     and #g1>0 and #g2>0
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)))
        let again = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && g1.len() > 0
            && g2.len() > 0
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3));
        if !again {
            break;
        }
    }
}
