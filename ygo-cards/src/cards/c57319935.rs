//! Xyz Xtreme !!
use super::prelude::*;

pub const ID: u32 = 57319935;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsPreviousControler(tp)
    card::is_type(d, c, TYPE_XYZ) && card::is_previous_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg).nil();
    // if s.cfilter(tc,tp) then
    if cfilter(d, tc, a.tp) {
        // e:SetLabel(tc:GetRank())
        let rank = card::get_rank(d, tc) as i64;
        effect::set_label(d, a.e, &[rank]);
        // return true
        return true;
    }
    // tc=eg:GetNext()
    let next = group::iter(d, &eg).get(1).copied();
    // if tc and s.cfilter(tc,tp) then
    if let Some(tc) = next {
        if cfilter(d, tc, a.tp) {
            // e:SetLabel(tc:GetRank())
            let rank = card::get_rank(d, tc) as i64;
            effect::set_label(d, a.e, &[rank]);
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsRankBelow,tp,LOCATION_EXTRA,0,1,nil,e:GetLabel()) end
    if a.chk == Some(0) {
        let rank = effect::get_label(d, a.e) as u32;
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_rank_below(d, c, rank)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let rank = effect::get_label(d, a.e) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local tc1=Duel.SelectMatchingCard(tp,Card.IsRankBelow,tp,LOCATION_EXTRA,0,1,1,nil,e:GetLabel()):GetFirst()
    let sel1 = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_rank_below(d, c, rank)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &sel1);
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, aux::stringid(ID, 0));
    // local tc2=Duel.SelectMatchingCard(1-tp,Card.IsRankBelow,1-tp,LOCATION_EXTRA,0,1,1,nil,e:GetLabel()):GetFirst()
    let sel2 = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| card::is_rank_below(d, c, rank)), 1 - a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc2 = group::get_first(d, &sel2);
    match (tc1, tc2) {
        // if tc1 and tc2 then
        (Some(tc1), Some(tc2)) => {
            // Duel.ConfirmCards(1-tp,tc1)
            duel::confirm_cards(d, 1 - a.tp, tc1);
            // Duel.ConfirmCards(tp,tc2)
            duel::confirm_cards(d, a.tp, tc2);
            // local atk1=tc1:GetAttack()
            let atk1 = card::get_attack(d, tc1) as i64;
            // local atk2=tc2:GetAttack()
            let atk2 = card::get_attack(d, tc2) as i64;
            // if atk1>atk2 then
            if atk1 > atk2 {
                // Duel.Damage(1-tp,atk1-atk2,REASON_EFFECT)
                duel::damage(d, 1 - a.tp, atk1 - atk2, REASON_EFFECT, false, None);
            // elseif atk1<atk2 then
            } else if atk1 < atk2 {
                // Duel.Damage(tp,atk2-atk1,REASON_EFFECT)
                duel::damage(d, a.tp, atk2 - atk1, REASON_EFFECT, false, None);
            }
        }
        // elseif tc1 then
        (Some(tc1), None) => {
            // Duel.ConfirmCards(1-tp,tc1)
            duel::confirm_cards(d, 1 - a.tp, tc1);
            // Duel.Damage(1-tp,tc1:GetAttack(),REASON_EFFECT)
            let atk = card::get_attack(d, tc1) as i64;
            duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
        }
        // elseif tc2 then
        (None, Some(tc2)) => {
            // Duel.ConfirmCards(tp,tc2)
            duel::confirm_cards(d, a.tp, tc2);
            // Duel.Damage(tp,tc2:GetAttack(),REASON_EFFECT)
            let atk = card::get_attack(d, tc2) as i64;
            duel::damage(d, a.tp, atk, REASON_EFFECT, false, None);
        }
        (None, None) => {}
    }
}
