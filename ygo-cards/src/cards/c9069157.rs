//! Dragonroid
use super::prelude::*;

pub const ID: u32 = 9069157;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 non-WIND "roid" monster or prevent your Fusion Summoning effects from getting negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card become a Dragon monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.rctg)
    effect::set_target(d, e2, Some(cost_target(rctg)));
    // e2:SetOperation(s.rcop)
    effect::set_operation(d, e2, Some(operation(rcop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ROID) and c:IsAttributeExcept(ATTRIBUTE_WIND) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ROID as u16]) && card::is_attribute_except(d, c, ATTRIBUTE_WIND, None, 0, None) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=true
    let b2 = true;
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    } else {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --inactivatable
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_INACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_INACTIVATE);
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(efilter));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // --act limit
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
        // e2:SetCondition(s.limcon)
        effect::set_condition(d, e2, Some(condition(limcon)));
        // e2:SetOperation(s.limop)
        effect::set_operation(d, e2, Some(operation(limop)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_CHAIN_END)
        effect::set_code(d, e3, EVENT_CHAIN_END);
        // e3:SetOperation(s.limop2)
        effect::set_operation(d, e3, Some(operation(limop2)));
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
// function s.efilter(e,ct)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local p=e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, a.e(0));
    // local te,tp=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
    let ct = a.int(1) as u8;
    let v = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = v[0].as_effect();
    let tp = v[1].int();
    // return p==tp and te:IsHasCategory(CATEGORY_FUSION_SUMMON)
    p as i64 == tp && effect::is_has_category(d, te.nil(), CATEGORY_FUSION_SUMMON)
}
// function s.limfilter(c,tp)
fn limfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsFusionSummoned()
    card::is_summon_player(d, c, tp) && card::is_fusion_summoned(d, c)
}
// function s.limcon(e,tp,eg,ep,ev,re,r,rp)
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.limfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| limfilter(d, c, a.tp), 1, NO_EX)
}
// function s.limop(e,tp,eg,ep,ev,re,r,rp)
fn limop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetCurrentChain()==0 then
    if duel::get_current_chain(d, false) == 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    // elseif Duel.GetCurrentChain()==1 then
    } else if duel::get_current_chain(d, false) == 1 {
        let h = effect::get_handler(d, a.e).nil();
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAINING)
        effect::set_code(d, e1, EVENT_CHAINING);
        // e1:SetOperation(s.resetop)
        effect::set_operation(d, e1, Some(operation(resetop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EVENT_BREAK_EFFECT)
        effect::set_code(d, e2, EVENT_BREAK_EFFECT);
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
// function s.resetop(e,tp,eg,ep,ev,re,r,rp)
fn resetop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
    // e:Reset()
    effect::reset(d, a.e);
}
// function s.limop2(e,tp,eg,ep,ev,re,r,rp)
fn limop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():HasFlagEffect(id) then
    if card::has_flag_effect(d, h, ID, None) {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
    // e:GetHandler():ResetFlagEffect(id)
    card::reset_flag_effect(d, h, ID);
}
// function s.chainlm(e,rp,tp)
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.int(2) == a.int(1)
}
// function s.rctg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsRace(RACE_DRAGON) end
    if a.chk == Some(0) {
        return !card::is_race(d, c, RACE_DRAGON);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.rcop(e,tp,eg,ep,ev,re,r,rp)
fn rcop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
    // e1:SetValue(RACE_DRAGON)
    effect::set_value(d, e1, RACE_DRAGON as i64);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
