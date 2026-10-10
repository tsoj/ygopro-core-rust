//! Bottomless Trap Hole
use super::prelude::*;

pub const ID: u32 = 29401950;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);

    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(activate)));
    card::register_effect(d, c, e2, false);

    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e3, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e3, Some(cost_target(target2)));
    effect::set_operation(d, e3, Some(operation(activate2)));
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8, ep: u8) -> bool {
    // c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:GetAttack()>=1500 and ep~=tp and c:IsAbleToRemove()
    card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::get_attack(d, c) >= 1500 && ep != tp && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil());
    // if chk==0 then return s.filter(tc,tp,ep) end
    if a.chk == Some(0) {
        return filter(d, tc.nil(), a.tp, a.ep);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg());
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:GetAttack()>=1500 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::get_attack(d, tc) >= 1500 {
            // Duel.Destroy(tc,REASON_EFFECT,LOCATION_REMOVED)
            duel::destroy(d, tc, REASON_EFFECT, Some(LOCATION_REMOVED), None);
        }
    }
}
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:GetAttack()>=1500 and c:GetSummonPlayer()~=tp and c:IsAbleToRemove()
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::get_attack(d, c) >= 1500
        && card::get_summon_player(d, c) != tp
        && card::is_able_to_remove(d, c, None, None, None)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:IsExists(s.filter2,1,nil,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter2(d, c, a.tp), 1, NO_EX);
    }
    // local g=eg:Filter(s.filter2,nil,tp)
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| filter2(d, c, a.tp), NO_EX);
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsFaceup() and c:GetAttack()>=1500 and not c:IsSummonPlayer(tp) and c:IsRelateToEffect(e) and c:IsLocation(LOCATION_MZONE)
    card::is_faceup(d, c)
        && card::get_attack(d, c) >= 1500
        && !card::is_summon_player(d, c, tp)
        && card::is_relate_to_effect(d, c, e)
        && card::is_location(d, c, LOCATION_MZONE)
}
fn activate2(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter3,nil,e,tp)
    let g = group::filter(d, &a.eg(), &|d, c| filter3(d, c, a.e, a.tp), NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT,LOCATION_REMOVED) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, Some(LOCATION_REMOVED), None);
    }
}
