//! Black Whirlwind
//! 黒い旋風
use super::prelude::*;

pub const ID: u32 = 91351370;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,val)
fn filter(d: &mut Duel, c: CardId, val: i32) -> bool {
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c);
    // return c:IsAttackAbove(0) and atk<val and c:IsSetCard(SET_BLACKWING) and c:IsMonster() and c:IsAbleToHand()
    card::is_attack_above(d, c, 0) && atk < val && card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if chk==0 then return tc:IsSetCard(SET_BLACKWING) and tc:IsControler(tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tc:GetAttack()) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        if !card::is_set_card(d, tc, &[SET_BLACKWING as u16]) || !card::is_controler(d, tc, tp) {
            return false;
        }
        let atk = card::get_attack(d, tc);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, atk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc.nil(), a.e);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if not e:GetHandler():IsRelateToEffect(e) or not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tc:GetAttack())
    let atk = card::get_attack(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, atk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
