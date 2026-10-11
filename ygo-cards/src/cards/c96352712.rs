//! Overexaggeration
use super::prelude::*;

pub const ID: u32 = 96352712;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dc=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // if chk==0 then return dc~=nil end
    if a.chk == Some(0) {
        return dc.is_some();
    }
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d).nil();
    // local b1=ac:IsAttackPos() and ac:IsCanChangePosition()
    let b1 = card::is_attack_pos(d, ac) && card::is_can_change_position(d, ac);
    // local b2=dc:IsAbleToHand()
    let dc = dc.nil();
    let b2 = card::is_able_to_hand(d, dc, None);
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {true,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (true, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_POSITION)
        effect::set_category(d, a.e, CATEGORY_POSITION);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,ac,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(ac)), 1, 0, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,dc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(dc)), 1, 0, 0);
    // elseif op==3 then
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Group.FromCards(ac,dc),2,0,0)
        let mut g = CardSet::from_card(ac);
        group::add_card(&mut g, &CardSet::from_card(dc));
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    }
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op.unwrap_or(0) as u32);
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, _a: &EffArgs) {
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // local dc=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // if op==1 then
    if op == 1 {
        // Change the attacking monster to Defense Position
        // if ac:IsAttackPos() and ac:IsRelateToBattle() then
        if card::is_attack_pos(d, ac.nil()) && card::is_relate_to_battle(d, ac.nil()) {
            // Duel.ChangePosition(ac,POS_FACEUP_DEFENSE)
            duel::change_position(d, ac.nil(), POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Return the attack target to the hand
        // if dc and dc:IsRelateToBattle() then
        if let Some(dc) = dc {
            if card::is_relate_to_battle(d, dc) {
                // Duel.SendtoHand(dc,nil,REASON_EFFECT)
                duel::send_to_hand(d, dc, None, REASON_EFFECT, None);
            }
        }
    // elseif op==3 then
    } else if op == 3 {
        // Destroy both battling monsters
        // local g=Group.FromCards(ac,dc):Match(Card.IsRelateToBattle,nil)
        let mut g = CardSet::from_card(ac.nil());
        if let Some(dc) = dc {
            group::add_card(&mut g, &CardSet::from_card(dc));
        }
        group::match_(d, &mut g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
        // if #g~=2 then return end
        if g.len() != 2 {
            return;
        }
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
