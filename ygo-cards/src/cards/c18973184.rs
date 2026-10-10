//! Branded Lost
use super::prelude::*;

pub const ID: u32 = 18973184;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot negate the activation of your Fusion Summoning cards/effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot activate cards/effects when Fusion Summoning
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.limcon)
    effect::set_condition(d, e3, Some(condition(limcon)));
    // e3:SetOperation(s.limop)
    effect::set_operation(d, e3, Some(operation(limop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e4, EVENT_CHAIN_END);
    // e4:SetOperation(s.limop2)
    effect::set_operation(d, e4, Some(operation(limop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Search
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.thcon)
    effect::set_condition(d, e5, Some(condition(thcon)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.efilter(e,ct)
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ct = a.int(1) as u8;
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local te,rp=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = info[0].as_effect();
    // return tp==rp and te:IsHasCategory(CATEGORY_FUSION_SUMMON)
    if matches!(info[1], Value::Nil) || info[1].int() as u8 != tp {
        return false;
    }
    effect::is_has_category(d, te.nil(), CATEGORY_FUSION_SUMMON)
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
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| limfilter(d, c, tp), 1, NO_EX)
}
// function s.chainlm(e,rp,tp)
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
// function s.limop(e,tp,eg,ep,ev,re,r,rp)
fn limop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetCurrentChain()==0 then
    if duel::get_current_chain(d, false) == 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    } else if duel::get_current_chain(d, false) == 1 {
        let c = effect::get_handler(d, a.e).nil();
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
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
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, ID);
    // e:Reset()
    effect::reset(d, a.e);
}
// function s.limop2(e,tp,eg,ep,ev,re,r,rp)
fn limop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetFlagEffect(id)>0 then
    if card::get_flag_effect(d, c, ID) > 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
    // e:GetHandler():ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsFusionSummoned() and c:IsType(TYPE_FUSION)
    card::is_summon_player(d, c, tp) && card::is_fusion_summoned(d, c) && card::is_type(d, c, TYPE_FUSION)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_ALBAZ) or (c:IsMonster() and c:ListsCode(CARD_ALBAZ))) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_ALBAZ])))
        && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
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
}
