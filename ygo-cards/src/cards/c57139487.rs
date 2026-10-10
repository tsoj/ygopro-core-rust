//! Chain Disappearance
use super::prelude::*;

pub const ID: u32 = 57139487;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:GetAttack()<=1000 and c:IsAbleToRemove()
    card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::get_attack(d, c) <= 1000 && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:IsExists(s.filter,1,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &filter, 1, NO_EX);
    }
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &filter, NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.efilter(c,e)`
fn efilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackBelow(1000)
    card::is_faceup(d, c) && card::is_attack_below(d, c, 1000)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetTargetCards(e):Filter(s.efilter,nil)
    let tg = duel::get_target_cards(d, a.e).nil();
    let sg = group::filter(d, &tg, &efilter, NO_EX);
    // if #sg==0 or Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)==0 then return end
    if sg.is_empty() || duel::remove(d, &sg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local rg=Group.CreateGroup()
    let mut rg = CardSet::new();
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // if tc:IsLocation(LOCATION_REMOVED) then
        if card::is_location(d, tc, LOCATION_REMOVED) {
            // local tpe=tc:GetType()
            let tpe = card::get_type(d, tc);
            // if (tpe&TYPE_TOKEN)==0 then
            if (tpe & TYPE_TOKEN) == 0 {
                // rg:Merge(Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_DECK|LOCATION_HAND,nil,tc:GetCode()))
                let codes = card::get_code(d, tc);
                let m = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &codes)), a.tp, 0, LOCATION_DECK | LOCATION_HAND, NO_EX);
                group::add_card(&mut rg, &m);
            }
        }
    }
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, &rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
