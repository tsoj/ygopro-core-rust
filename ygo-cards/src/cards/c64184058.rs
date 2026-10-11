//! Destiny HERO - Decider
use super::prelude::*;

pub const ID: u32 = 64184058;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --cannot be battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, lua_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --neg damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_F);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e4, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e4:SetCondition(aux.damcon1)
    effect::set_condition(d, e4, Some(aux::damcon1()));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.atlimit(e,c)
fn atlimit(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    let c = a.c(1);
    // return c:IsControler(1-tp) and c:IsLevelAbove(6) and not c:IsImmuneToEffect(e)
    card::is_controler(d, c, 1 - tp) && card::is_level_above(d, c, 6) && !card::is_immune_to_effect(d, c, a.e(0))
}
// s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HERO) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
// s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()  (read in the Lua before the check: only used after it)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None) != 0 {
        // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
        let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetLabel(cid)
        effect::set_label(d, e1, &[cid]);
        // e1:SetValue(s.damval)
        effect::set_value_fn(d, e1, lua_fn(damval));
        // e1:SetReset(RESET_CHAIN)
        effect::set_reset(d, e1, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// s.damval(e,re,val,r,rp,rc)
fn damval(d: &mut Duel, a: Args) -> i64 {
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if cc==0 or (r&REASON_EFFECT)==0 then return val end
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return val;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // if cid~=e:GetLabel() then return val end
    if cid != effect::get_label(d, a.e(0)) {
        return val;
    }
    // return 0
    0
}
