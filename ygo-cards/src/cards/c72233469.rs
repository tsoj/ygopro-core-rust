//! リィラップ
//! Lil-la Rap
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 72233469;

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
    // Opponent loses LP and you gain LP
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.lpcon)
    effect::set_condition(d, e2, Some(condition(lpcon)));
    // e2:SetTarget(s.lptg)
    effect::set_target(d, e2, Some(cost_target(lptg)));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsSummonLocation(LOCATION_GRAVE)
    card::is_summon_player(d, c, tp) && card::is_summon_location(d, c, LOCATION_GRAVE)
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.lptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 500);
    true
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.SetLP(1-tp,Duel.GetLP(1-tp)-1000)
    let lp = duel::get_lp(d, 1 - tp);
    duel::set_lp(d, 1 - tp, lp - 1000);
    // Duel.Recover(tp,500,REASON_EFFECT)
    duel::recover(d, tp, 500, REASON_EFFECT, false, None);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,nil,aux.Stringid(id,1))
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
}
