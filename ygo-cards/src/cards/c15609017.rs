//! Shock Surprise
use super::prelude::*;

pub const ID: u32 = 15609017;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_SPEEDROID as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local ct=#sg
    let ct = sg.len();
    let e = e.nil();
    // sg:AddCard(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let mut sg2 = sg.clone();
    group::add_card(&mut sg2, &CardSet::from_card(h));
    // local res=Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,sg)
    let res = duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, (&sg2).into());
    // sg:RemoveCard(e:GetHandler())
    // return res
    (res, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rg=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    let rs: aux::Rescon = &rescon;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, a.e, &[0]);
            // return aux.SelectUnselectGroup(rg,e,tp,nil,2,s.rescon,0)
            return matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, None, Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
        } else {
            return false;
        }
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local rsg=aux.SelectUnselectGroup(rg,e,tp,nil,2,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
    let rsg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, None, Some(2), Some(rs), 1, a.tp, Some(HINTMSG_REMOVE as u64), Some(rs), None, false)
        .unwrap_or_default();
    // local ct=Duel.Remove(rsg,POS_FACEUP,REASON_COST)
    let ct = duel::remove(d, rsg, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,e:GetHandler())
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
