//! Gilford the Legend
use super::prelude::*;

pub const ID: u32 = 69933858;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c,g)`: `return c:IsType(TYPE_EQUIP) and g:IsExists(s.eqcheck,1,nil,c)`
fn eqfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    card::is_type(d, c, TYPE_EQUIP) && group::is_exists(d, g, &|d, x| eqcheck(d, x, c), 1, NO_EX)
}
/// `s.eqcheck(c,ec)`: `return ec:CheckEquipTarget(c)`
fn eqcheck(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::check_equip_target(d, ec, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return false end
        if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)), a.tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,g)
        return duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, &g)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local eq=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_GRAVE,0,nil,g)
    let mut eq = duel::get_matching_group(d, Some(&|d, c| eqfilter(d, c, &g)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if ft>#eq then ft=#eq end
    if ft as usize > eq.len() {
        ft = eq.len() as i32;
    }
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // for i=1,ft do
    for _i in 1..=ft {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local ec=eq:Select(tp,1,1,nil):GetFirst()
        let ec = group::select(d, &eq, a.tp, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &ec).nil();
        // eq:RemoveCard(ec)
        group::remove_card(&mut eq, &CardSet::from_card(ec));
        // local tc=g:FilterSelect(tp,s.eqcheck,1,1,nil,ec):GetFirst()
        let tc = group::filter_select(d, &g, a.tp, &|d, c| eqcheck(d, c, ec), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &tc).nil();
        // Duel.Equip(tp,ec,tc,true,true)
        duel::equip(d, a.tp, ec, tc, true, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
