//! Battleguard Howling
use super::prelude::*;

pub const ID: u32 = 78621186;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e2, EVENT_BECOME_TARGET);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition1(e,tp,eg,ep,ev,re,r,rp)
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // return tc:IsFaceup() and tc:IsControler(tp) and tc:IsRace(RACE_WARRIOR)
    card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) && card::is_race(d, tc, RACE_WARRIOR)
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsRace(RACE_WARRIOR)
    card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_race(d, c, RACE_WARRIOR)
}
// function s.condition2(e,tp,eg,ep,ev,re,r,rp)
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and re:IsMonsterEffect() and eg:IsExists(s.cfilter,1,nil,tp)
    a.rp != a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local dam=tc:GetBaseAttack()
        let mut dam = card::get_base_attack(d, tc) as i64;
        // if dam<0 then dam=0 end
        if dam < 0 {
            dam = 0;
        }
        // if Duel.Damage(1-tp,dam,REASON_EFFECT)==0 then return end
        if duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None) == 0 {
            return;
        }
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
