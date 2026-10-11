//! R－ライトジャスティス (R - Righteous Justice)
use super::prelude::*;

pub const ID: u32 = 37318031;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ELEMENTAL_HERO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // e:SetLabel(ct)
        effect::set_label(d, a.e, &[ct as i64]);
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,c)
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u32, c.into());
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local sg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let sg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), ct as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if #g>=ct then
    if g.len() >= ct {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,ct,ct,nil)
        let sg = group::select(d, &g, tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
