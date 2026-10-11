//! Frodi, Generaider Boss of Swords
//! 剣の王 フローディ
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 40998517;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
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
    // return c:IsSetCard(SET_GENERAIDER) or c:IsRace(RACE_WARRIOR)
    card::is_set_card(d, c, &[SET_GENERAIDER as u16]) || card::is_race(d, c, RACE_WARRIOR as u64)
}
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, _mg: &CardSet) -> (bool, bool) {
    // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,#sg,sg)
    (duel::is_existing_target(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, sg.len() as u16, sg.into()), false)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.spcheck,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&spcheck), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,99,false,s.spcheck,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 99, false, Some(&spcheck), NO_EX);
    // local ct=Duel.Release(g,REASON_COST)
    let ct = duel::release(d, g, REASON_COST, None);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,ct,ct,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, ct, ct, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, ct as u8, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
        // local g=Duel.GetOperatedGroup():Match(Card.IsPreviousControler,nil,1-tp)
        let og = duel::get_operated_group(d);
        let g = group::filter(d, &og, &|d, c| card::is_previous_controler(d, c, 1 - a.tp), NO_EX);
        // if #g>0 and Duel.IsPlayerCanDraw(1-tp,#g) and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::is_player_can_draw(d, 1 - a.tp, g.len() as u32) && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(1-tp,#g,REASON_EFFECT)
            duel::draw(d, 1 - a.tp, g.len() as u32, REASON_EFFECT);
        }
    }
}
