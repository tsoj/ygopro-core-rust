//! Smile Action
//! スマイル・アクション
use super::prelude::*;

pub const ID: u32 = 47870325;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
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
    // Negate attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToRemove(POS_FACEDOWN)
    // (the Lua passes POS_FACEDOWN as the player argument of IsAbleToRemove; kept as written)
    card::is_spell(d, c) && card::is_able_to_remove(d, c, Some(POS_FACEDOWN as u8), None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local rg=Group.CreateGroup()
    let mut rg = CardSet::new();
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local step=turn_p==0 and 1 or -1
    // for p=turn_p,1-turn_p,step do
    for p in [turn_p, 1 - turn_p] {
        // if Duel.IsExistingMatchingCard(s.rmfilter,p,LOCATION_GRAVE,0,1,nil) and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&rmfilter), p, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, p, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_REMOVE as u64);
            // local g=Duel.SelectMatchingCard(p,s.rmfilter,p,LOCATION_GRAVE,0,1,5,nil)
            let g = duel::select_matching_card(d, p, Some(&rmfilter), p, LOCATION_GRAVE, 0, 1, 5, false, NO_EX).nil();
            // rg:Merge(g)
            group::add_card(&mut rg, &g);
        }
    }
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // Duel.HintSelection(rg,true)
    duel::hint_selection(d, rg.clone(), true);
    // if Duel.Remove(rg,POS_FACEDOWN,REASON_EFFECT)>0 then
    if duel::remove(d, rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // for tc in og:Iter() do
        for tc in group::iter(d, &og) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsAbleToHand()
    card::get_flag_effect(d, c, ID) != 0 && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local p=1-at:GetControler()
    let p = 1 - card::get_controler(d, at);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,p,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, p, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,1,p,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 1, p, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local p=1-at:GetControler()
    let p = 1 - card::get_controler(d, at);
    // if Duel.IsExistingMatchingCard(s.thfilter,p,LOCATION_REMOVED,0,1,nil) and Duel.SelectYesNo(p,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&thfilter), p, LOCATION_REMOVED, 0, 1, NO_EX)
        && duel::select_yes_no(d, p, aux::stringid(ID, 2))
    {
        // local g=Duel.GetMatchingGroup(s.thfilter,p,LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), p, LOCATION_REMOVED, 0, NO_EX);
        // local tg=g:RandomSelect(p,1)
        let tg = group::random_select(d, &g, p, 1);
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
        // local tc=tg:GetFirst()
        let tc = group::get_first(d, &tg).nil();
        // if tc:IsDiscardable(REASON_EFFECT) and Duel.SelectYesNo(p,aux.Stringid(id,3)) then
        if card::is_discardable(d, tc, Some(REASON_EFFECT), None) && duel::select_yes_no(d, p, aux::stringid(ID, 3)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT | REASON_DISCARD, None, None);
            // Duel.NegateAttack()
            duel::negate_attack(d);
        } else {
            // --double damage
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
            effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetValue(DOUBLE_DAMAGE)
            effect::set_value(d, e1, DOUBLE_DAMAGE as i64);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,p)
            duel::register_effect(d, e1, p);
        }
    }
}
