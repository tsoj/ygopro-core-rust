//! Photon Change
use super::prelude::*;

pub const ID: u32 = 42925441;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GALAXYEYES_P_DRAGON],
    listed_series: &[SET_PHOTON, SET_GALAXY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects, or, if you send "Galaxy-Eyes Photon Dragon", you can activate both, in sequence
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.effcost)
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Send this card to the GY during your 2nd Standby Phase after activation.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,3)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 3);
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if s.effcost(e,tp,eg,ep,ev,re,r,rp,0) and s.efftg(e,tp,eg,ep,ev,re,r,rp,0)
    //     and Duel.SelectYesNo(tp,94) then
    if effcost_chk(d, a.e, a.tp, true) && efftg_chk(d, a.e, a.tp, true) && duel::select_yes_no(d, a.tp, 94) {
        // s.effcost(e,tp,eg,ep,ev,re,r,rp,1)
        effcost_chk(d, a.e, a.tp, false);
        // s.efftg(e,tp,eg,ep,ev,re,r,rp,1)
        efftg_chk(d, a.e, a.tp, false);
        // e:SetOperation(s.effop)
        effect::set_operation(d, a.e, Some(operation(effop)));
    }
    true
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    // ct=ct+1
    let ct = card::get_turn_counter(d, c) + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==2 then
    if ct == 2 {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    effcost_chk(d, a.e, a.tp, a.chk == Some(0))
}
/// The body of `s.effcost`: `chk0` is `chk==0`.
fn effcost_chk(d: &mut Duel, _e: EffectId, tp: u8, chk0: bool) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if chk0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESETS_STANDARD_PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0);
    true
}
// function s.costfilter1(c,e,tp)
fn costfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and c:IsFaceup() and c:IsSetCard({SET_PHOTON,SET_GALAXY}) and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetOriginalCode())
    if !(card::is_able_to_grave_as_cost(d, c)
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16])
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
    {
        return false;
    }
    let code = card::get_original_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter1(c,e,tp,code)
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_PHOTON) and c:GetOriginalCode()~=code and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_PHOTON as u16])
        && card::get_original_code(d, c) != code
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.costfilter2(c)
fn costfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_PHOTON,SET_GALAXY}) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16]) && card::is_able_to_grave_as_cost(d, c)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PHOTON) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PHOTON as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.costfilter3(c,e,tp)
fn costfilter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and c:IsFaceup() and c:IsCode(CARD_GALAXYEYES_P_DRAGON) and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetOriginalCode())
    if !(card::is_able_to_grave_as_cost(d, c)
        && card::is_faceup(d, c)
        && card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON])
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
    {
        return false;
    }
    let code = card::get_original_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter2(c,e,tp,code)
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_PHOTON) and c:GetOriginalCode()~=code and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c)
    card::is_set_card(d, c, &[SET_PHOTON as u16])
        && card::get_original_code(d, c) != code
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    efftg_chk(d, a.e, a.tp, a.chk == Some(0))
}
/// The body of `s.efftg`: `chk0` is `chk==0`.
fn efftg_chk(d: &mut Duel, e: EffectId, tp: u8, chk0: bool) -> bool {
    // local b1=Duel.IsExistingMatchingCard(s.costfilter1,tp,LOCATION_MZONE,0,1,nil,e,tp)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| costfilter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.costfilter2,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&costfilter2), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(s.costfilter3,tp,LOCATION_MZONE,0,1,nil,e,tp)
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| costfilter3(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if chk0 {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)}, {b3,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3)), (b3, aux::stringid(ID, 4))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // e:SetValue(g:GetFirst():GetOriginalCode())
        let tc = group::get_first(d, &g).nil();
        let code = card::get_original_code(d, tc);
        effect::set_value(d, e, code as i64);
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter2,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&costfilter2), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else if op == Some(3) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter3,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter3(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // e:SetValue(g:GetFirst():GetOriginalCode())
        let tc = group::get_first(d, &g).nil();
        let code = card::get_original_code(d, tc);
        effect::set_value(d, e, code as i64);
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetValue())
        let code = effect::get_value(d, e).int() as u32;
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter1(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    } else if op == 3 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetValue())
        let code = effect::get_value(d, e).int() as u32;
        let g1 = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g1>0 and Duel.SpecialSummon(g1,0,tp,tp,false,false,POS_FACEUP)~=0 then
        if !g1.is_empty() && duel::special_summon(d, g1, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g2=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
            let g2 = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g2>0 then
            if !g2.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g2,nil,REASON_EFFECT)
                duel::send_to_hand(d, &g2, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g2)
                duel::confirm_cards(d, 1 - tp, &g2);
            }
        }
    }
}
