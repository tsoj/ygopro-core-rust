//! D/D/D Sky King Zeus Ragnarok
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 30998403;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD, SET_DARK_CONTRACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ "D/D" monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DD),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_DD as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Destroy 1 "D/D" or "Dark Contract" card you control, also, you can conduct 1 Pendulum Summon of a "D/D" monster(s) in addition to your Pendulum Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of monster effect your opponent activates in the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(s.negcost)
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,{SET_DD,SET_DARK_CONTRACT})`
fn dd_dc_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DD as u16, SET_DARK_CONTRACT as u16])
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsSetCard({SET_DD,SET_DARK_CONTRACT}) and chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_set_card(d, chkc, &[SET_DD as u16, SET_DARK_CONTRACT as u16])
            && card::is_on_field(d, chkc)
            && card::is_controler(d, chkc, tp)
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,{SET_DD,SET_DARK_CONTRACT}),tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, tp, ID, None)
            && duel::is_existing_target(d, Some(&dd_dc_faceup), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,{SET_DD,SET_DARK_CONTRACT}),tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&dd_dc_faceup), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
    // Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),function(c) return c:IsSetCard(SET_DD) end,tp,LOCATION_HAND|LOCATION_EXTRA,aux.Stringid(id,2),aux.Stringid(id,3),id)
    let h = effect::get_handler(d, a.e).nil();
    let filter: pendulum::PendFilter = std::sync::Arc::new(|d, cc, _e, _tp, _a, _b| card::is_set_card(d, cc, &[SET_DD as u16]));
    pendulum::grant_additional_pendulum_summon(
        d,
        h,
        Some(filter),
        tp,
        LOCATION_HAND | LOCATION_EXTRA,
        Some(aux::stringid(ID, 2)),
        Some(aux::stringid(ID, 3)),
        ID,
        None,
        None,
    );
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ev = a.ev as u8;
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, ev, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return ep==1-tp and re:IsMonsterEffect() and trig_loc==LOCATION_HAND
    //     and Duel.IsChainNegatable(ev) and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    if a.ep != 1 - tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if trig_loc != LOCATION_HAND as i64 {
        return false;
    }
    if !duel::is_chain_negatable(d, ev) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
/// `s.negcostfilter(c)`
fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_DARK_CONTRACT) or (c:IsSetCard(SET_DD) and c:IsMonster())) and c:IsAbleToRemoveAsCost()
    (card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16]) || (card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c)))
        && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_DARK_CONTRACT)
    //     and sg:IsExists(function(c) return c:IsSetCard(SET_DD) and c:IsMonster() end,1,nil)
    let ok = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c), 1, NO_EX);
    (ok, false)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.negcostfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&negcostfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
