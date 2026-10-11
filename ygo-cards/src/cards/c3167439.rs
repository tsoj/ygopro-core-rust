//! Dark World Punishment
//! 暗黒界の懲罰
use super::prelude::*;

pub const ID: u32 = 3167439;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate Normal or Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.GetCurrentChain(true)==0 end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::get_current_chain(d, true) == 0)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Replace destruction
    // local e2=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e3, lua_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsDiscardable(REASON_EFFECT)
    card::is_race(d, c, RACE_FIEND) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, eg.clone());
    // if Duel.Destroy(eg,REASON_EFFECT)==0 then return end
    if duel::destroy(d, eg, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_DARK_WORLD) and c:IsControler(tp) and not c:IsReason(REASON_REPLACE)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_DARK_WORLD as u16])
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, tp, c, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    repfilter(d, a.c(1), effect::get_handler_player(d, a.e(0)))
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
