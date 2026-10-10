//! Magical Meltdown
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86120751], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 47679935;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // inactivatable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // act limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.limcon)
    effect::set_condition(d, e3, Some(condition(limcon)));
    // e3:SetOperation(s.limop)
    effect::set_operation(d, e3, Some(operation(limop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e5, EVENT_CHAIN_END);
    // e5:SetOperation(s.limop2)
    effect::set_operation(d, e5, Some(operation(limop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(86120751) and c:IsAbleToHand()
    card::is_code(d, c, &[86120751]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ct = a.int(1);
    // local p=e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, e);
    // local te,tp=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
    let info = duel::get_chain_info(d, ct as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = info.first().and_then(|v| v.as_effect());
    let tp = info.get(1).cloned().unwrap_or_default();
    // return p==tp and te:IsHasCategory(CATEGORY_FUSION_SUMMON)
    if tp.is_nil() || p as i64 != tp.int() {
        return false;
    }
    effect::is_has_category(d, te.nil(), CATEGORY_FUSION_SUMMON)
}
/// `s.limfilter(c,tp)`
fn limfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsFusionSummoned()
    card::is_summon_player(d, c, tp) && card::is_fusion_summoned(d, c)
}
/// `s.limcon(e,tp,eg,ep,ev,re,r,rp)`
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.limfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| limfilter(d, c, tp), 1, NO_EX)
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetCurrentChain()==0 then
    if duel::get_current_chain(d, false) == 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
    // elseif Duel.GetCurrentChain()==1 then
    else if duel::get_current_chain(d, false) == 1 {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, a.e).nil();
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
        duel::register_effect(d, e1, tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EVENT_BREAK_EFFECT)
        effect::set_code(d, e2, EVENT_BREAK_EFFECT);
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.resetop(e,tp,eg,ep,ev,re,r,rp)`
fn resetop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.limop2(e,tp,eg,ep,ev,re,r,rp)`
fn limop2(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetFlagEffect(id)~=0 then
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
    // e:GetHandler():ResetFlagEffect(id)
    card::reset_flag_effect(d, h, ID);
}
