//! Infinite Machine
use super::prelude::*;

pub const ID: u32 = 36894320;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIMELORD], listed_names: &[9409625, 72883039], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.actcost)
    effect::set_cost(d, e1, Some(cost_target(actcost)));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_SET)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_SET);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCost(s.cost)
    effect::set_cost(d, e4, Some(cost_target(cost)));
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp and e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != a.tp && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // return true
    true
}
/// `s.acfilter(c)`
fn acfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(36894320) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.actcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.acfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&acfilter), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.acfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&acfilter), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc) end
    if a.chkc().is_some() {
        return tdtg(d, a);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let chk0 = EffArgs { chk: Some(0), ..a.clone() };
    let chk1 = EffArgs { chk: Some(1), ..a.clone() };
    // local b1=s.spcon(e,tp,eg,ep,ev,re,r,rp) and s.cost(e,tp,eg,ep,ev,re,r,rp,0) and s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = spcon(d, a) && cost(d, &chk0) && sptg(d, &chk0);
    // local b2=s.cost(e,tp,eg,ep,ev,re,r,rp,0) and s.tdtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = cost(d, &chk0) && tdtg(d, &chk0);
    // if (b1 or b2) and Duel.SelectYesNo(tp,94) then
    if (b1 || b2) && duel::select_yes_no(d, tp, 94) {
        // local op=0
        let op = if b1 && b2 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
            duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
        } else if b1 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,0))
            duel::select_option(d, tp, true, &[aux::stringid(ID, 0)])
        } else {
            // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
            duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) + 1
        };
        if op == 0 {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
            // e:SetProperty(0)
            effect::set_property(d, a.e, 0, 0);
            // e:SetOperation(s.spop)
            effect::set_operation(d, a.e, Some(operation(spop)));
            // s.cost(e,tp,eg,ep,ev,re,r,rp,1)
            cost(d, &chk1);
            // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
            sptg(d, &chk1);
        } else {
            // e:SetCategory(CATEGORY_TODECK)
            effect::set_category(d, a.e, CATEGORY_TODECK);
            // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
            effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
            // e:SetOperation(s.tdop)
            effect::set_operation(d, a.e, Some(operation(tdop)));
            // s.cost(e,tp,eg,ep,ev,re,r,rp,1)
            cost(d, &chk1);
            // s.tdtg(e,tp,eg,ep,ev,re,r,rp,1)
            tdtg(d, &chk1);
        }
    }
    true
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(1002270280)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, 1002270280) == 0;
    }
    // c:RegisterFlagEffect(1002270280,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, 1002270280, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TIMELORD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TIMELORD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TIMELORD) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_TIMELORD as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(9409625) and c:IsSSetable()
    card::is_code(d, c, &[9409625]) && card::is_s_setable(d, c, false, None)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0 {
            // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
            let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                // local sc=g:Select(tp,1,1,nil):GetFirst()
                let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &sel).nil();
                // Duel.SSet(tp,sc)
                duel::s_set(d, tp, sc, None, true);
            }
        }
    }
}
