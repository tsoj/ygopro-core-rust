//! Inferno Tempest
use super::prelude::*;

pub const ID: u32 = 14391920;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
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
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and ev>=3000
    a.ep == a.tp && a.ev >= 3000
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and (c:IsLocation(LOCATION_DECK) or aux.SpElimFilter(c))
    card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_location(d, c, LOCATION_DECK) || aux::sp_elim_filter(d, c, false, false))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&filter),
            a.tp,
            LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // local sg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,nil)
    let sg = duel::get_matching_group(
        d,
        Some(&filter),
        a.tp,
        LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
        LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,LOCATION_DECK|LOCATION_MZONE|LOCATION_GRAVE,nil)
    let sg = duel::get_matching_group(
        d,
        Some(&filter),
        a.tp,
        LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
        LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE,
        NO_EX,
    );
    // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
