//! Adamancipator Relief
use super::prelude::*;

pub const ID: u32 = 9341993;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ADAMANCIPATOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Tribute any number of "Adamacia" monsters, destroy that many cards +1
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ADAMANCIPATOR) and c:IsMonster()
    card::is_set_card(d, c, &[SET_ADAMANCIPATOR as u16]) && card::is_monster(d, c)
}
/// `s.spcheck(sg,tp,exg,oc)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, oc: CardId) -> (bool, bool) {
    // local ex=sg:Clone()
    let mut ex = sg.clone();
    // ex:AddCard(oc)
    group::add_card(&mut ex, &CardSet::from_card(oc));
    // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,#sg+1,ex)
    let res = duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, (sg.len() + 1) as u16, (&ex).into());
    (res, false)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let check = |d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet| spcheck(d, sg, tp, h);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.spcheck,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,99,false,s.spcheck,nil,e:GetHandler())
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 99, false, Some(&check), NO_EX);
    // local ct=Duel.Release(g,REASON_COST)
    let ct = duel::release(d, g, REASON_COST, None);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && chkc != h;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // local ct=e:GetLabel()+1
    let ct = (effect::get_label(d, a.e) + 1) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,e:GetHandler())
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, ct as u8, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsRelateToEffect,nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &targets, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #tg>0 then Duel.Destroy(tg,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
