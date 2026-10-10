//! A・O・J リターンゼロ (Arms of Genex Return Zero)
use super::prelude::*;

pub const ID: u32 = 61775475;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 DARK Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Negate monster effect activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetCost(s.negcost)
    effect::set_cost(d, e1, Some(cost_target(negcost)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle up to 6 "Genex" monsters into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register Attributes used
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s.attr_list={}
        // s.attr_list[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s.attr_list[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
        // aux.AddValuesReset(function() s.attr_list[0]=0 s.attr_list[1]=0 end)
        aux::add_values_reset(d, reset_attr_list);
    });
}
/// `s.attr_list[tp]`: the attributes this player already used for the cost, as a bit mask
fn attr_list(d: &Duel, tp: u8) -> u32 {
    s_var(d, ID, tp as u64).int() as u32
}
/// The reset of `s.attr_list` at the end of the turn
fn reset_attr_list(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    //     and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    if a.rp != 1 - a.tp {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
/// `s.negcostfilter(c,tp,att)`
fn negcostfilter(d: &mut Duel, c: CardId, tp: u8, att: u32) -> bool {
    // return (c:GetAttribute()&att&~s.attr_list[tp])>0 and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    let a = card::get_attribute(d, c) & att & !attr_list(d, tp);
    a > 0 && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local att=re:GetHandler():GetAttribute()
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    let att = card::get_attribute(d, rh);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp,att) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| negcostfilter(d, c, tp, att)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp,att)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| negcostfilter(d, c, tp, att)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // s.attr_list[tp]=s.attr_list[tp]|g:GetFirst():GetAttribute()
    let first = group::get_first(d, &g).nil();
    let first_attr = card::get_attribute(d, first);
    let cur = attr_list(d, tp);
    set_s_var(d, ID, tp as u64, Value::Int((cur | first_attr) as i64));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            let eg = a.eg();
            duel::destroy(d, eg, REASON_EFFECT, None, None);
        }
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GENEX) and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// The attribute as a number for `aux.dpcheck(Card.GetAttribute)`
fn attr_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_attribute(d, c) as i64
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,6,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_TODECK)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(1),
        Some(6),
        Some(&aux::dpcheck(attr_value)),
        1,
        tp,
        Some(HINTMSG_TODECK as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_STZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, LOCATION_STZONE as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg==0 or Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if tg.is_empty() || duel::send_to_deck(d, tg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local ct=tg:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &tg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_STZONE,LOCATION_STZONE)
    let g = duel::get_field_group(d, tp, LOCATION_STZONE, LOCATION_STZONE);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=g:Select(tp,1,ct,nil)
    let dg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
