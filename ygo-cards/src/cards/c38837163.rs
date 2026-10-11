//! Akz, the Pumer
use super::prelude::*;

pub const ID: u32 = 38837163;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_BEASTWARRIOR as u64) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let _ = c;
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>1 and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && rg.len() > 1
        && matches!(
            aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, a.e(0), Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsDiscardable()
    card::is_race(d, c, RACE_BEASTWARRIOR as u64) && card::is_discardable(d, c, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| cfilter(d, c)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_facedown(d, c) {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, e) == 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(c:GetAttack()*2)
        let atk = card::get_attack(d, c);
        effect::set_value(d, e1, (atk * 2) as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
