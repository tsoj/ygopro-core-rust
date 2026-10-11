//! Chain Hole
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 9581215;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // and rp~=tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) and Duel.GetCurrentChain(true)>1
    if a.rp == a.tp {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    duel::get_current_chain(d, true) > 1
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.filter(c,cd)
fn filter(d: &mut Duel, c: CardId, cd: u32) -> bool {
    card::is_code(d, c, &[cd]) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.NegateEffect(ev) then return end
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_HAND|LOCATION_DECK,nil,re:GetHandler():GetOriginalCode())
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    let cd = card::get_original_code(d, rh);
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| filter(d, c, cd)),
        tp,
        0,
        LOCATION_HAND | LOCATION_DECK,
        NO_EX,
    );
    // local hg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let hg = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        LOCATION_HAND,
        NO_EX,
    );
    // local rc=0
    let mut rc: i32 = 0;
    // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(1-tp,1,1,nil)
        let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        // rc=Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        rc = duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // if rc==0 and #hg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if rc == 0 && !hg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local sg=hg:RandomSelect(tp,1)
        let sg = group::random_select(d, &hg, tp, 1);
        // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
