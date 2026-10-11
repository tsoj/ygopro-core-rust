//! Superheavy Samurai Steam Train King
use super::prelude::*;

pub const ID: u32 = 17775525;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SUPERHEAVY_SAMURAI),1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_SUPERHEAVY_SAMURAI),2,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_SUPERHEAVY_SAMURAI as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_SUPERHEAVY_SAMURAI as u16], sc, st, Some(tp)))),
        2,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // defense attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DEFENSE_ATTACK)
    effect::set_code(d, e1, EFFECT_DEFENSE_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_COST), None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local rt=Duel.GetTargetCount(nil,tp,0,LOCATION_ONFIELD,nil)
    let mut rt = duel::get_target_count(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if rt>2 then rt=2 end
    if rt > 2 {
        rt = 2;
    }
    // local cg=Duel.DiscardHand(tp,Card.IsDiscardable,1,rt,REASON_COST|REASON_DISCARD,nil)
    let cg = duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, Some(REASON_COST), None)), 1, rt as u16, REASON_COST | REASON_DISCARD, NO_EX);
    // e:SetLabel(cg)
    effect::set_label(d, a.e, &[cg as i64]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,ct,ct,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, ct as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let tg = tg[0].as_group().cloned().nil();
    // local rg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let rg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Destroy(rg,REASON_EFFECT)
        duel::destroy(d, rg, REASON_EFFECT, None, None);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToRemove()
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (g.len() * 200) as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // local ct=Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    let ct = duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // if ct>0 then Duel.Damage(1-tp,ct*200,REASON_EFFECT) end
    if ct > 0 {
        duel::damage(d, 1 - tp, (ct * 200) as i64, REASON_EFFECT, false, None);
    }
}
