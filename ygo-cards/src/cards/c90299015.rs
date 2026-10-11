//! Vampire Ghost
use super::prelude::*;

pub const ID: u32 = 90299015;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAMPIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Normal Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function()return Duel.IsMainPhase()end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(s.nscost)
    effect::set_cost(d, e2, Some(cost_target(nscost)));
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcostfilter(c)`
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsAbleToGraveAsCost() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_VAMPIRE as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c)
    let cg = duel::select_matching_card(d, a.tp, Some(&thcostfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(cg,REASON_COST)
    duel::send_to_grave(d, cg, REASON_COST, None, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsLevelAbove(4) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && card::is_level_above(d, c, 4) && card::is_able_to_hand(d, c, None)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsLevelBelow(2) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && card::is_level_below(d, c, 2) && card::is_able_to_grave(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let hg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #hg>0 and Duel.SendtoHand(hg,nil,REASON_EFFECT)>0 and hg:GetFirst():IsLocation(LOCATION_HAND) then
    if !hg.is_empty() && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0 {
        let first = group::get_first(d, &hg).nil();
        if card::is_location(d, first, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,hg)
            duel::confirm_cards(d, 1 - a.tp, hg);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SendtoGrave(g,REASON_EFFECT)
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from a cost with the same arguments
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    aux::bfgcost()
        .call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(chk),
            ],
        )
        .truthy()
}
/// `s.nscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nscost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0) and Duel.CheckLPCost(tp,500) end
    if a.chk == Some(0) {
        return bfgcost(d, a, 0) && duel::check_lp_cost(d, a.tp, 500);
    }
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,1)
    bfgcost(d, a, 1);
    // Duel.PayLPCost(tp,500)
    duel::pay_lp_cost(d, a.tp, 500);
    true
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        // Duel.Summon(tp,g:GetFirst(),true,nil)
        duel::summon(d, a.tp, first, true, None, 0, None);
    }
}
