//! The Huge Revolution is Over
use super::prelude::*;

pub const ID: u32 = 99188141;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE);
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
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if (not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE))
    //     or not Duel.IsChainNegatable(ev) then return false end
    if (!effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and tc+tg:FilterCount(Card.IsOnField,nil)-#tg>1
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        None => false,
        Some(info) => match info.op_cards {
            None => false,
            Some(tg) => {
                let on_field = group::filter_count(d, &tg, &|d, x| card::is_on_field(d, x), NO_EX) as i64;
                info.op_count as i64 + on_field - tg.len() as i64 > 1
            }
        },
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None) || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let p = card::get_controler(d, rc);
        let loc = card::get_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, p, loc as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let loc = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, loc as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Remove(re:GetHandler(),POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
