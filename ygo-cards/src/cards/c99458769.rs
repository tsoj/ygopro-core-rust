//! Reign-Beaux, Overlord of Dark World
use super::prelude::*;

pub const ID: u32 = 99458769;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card if it is discarded to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destroy all monsters OR all Spells/Traps your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND) and rp==1-tp
    //     and (r&(REASON_DISCARD|REASON_EFFECT))==(REASON_DISCARD|REASON_EFFECT)
    //     and e:GetHandler():IsPreviousControler(tp)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_HAND)
        && a.rp == 1 - a.tp
        && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
        && card::is_previous_controler(d, c, a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c1=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let c1 = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // local c2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let c2 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if (#c1+#c2)>0 then
    if c1.len() + c2.len() > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c1+c2,1,tp,0)
        let mut u = c1.clone();
        group::add_card(&mut u, &c2);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(u), 1, a.tp, 0);
    } else {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    }
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g1 = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0
    let b2 = !g2.is_empty();
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=nil
    // if (b1 and b2) then
    let op = if b1 && b2 {
        // op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
        duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else {
        // op=(b1 and 1 or 2)
        Some(if b1 { 1 } else { 2 })
    };
    // Duel.Destroy(op==1 and g1 or g2,REASON_EFFECT)
    let targets = if op == Some(1) { g1 } else { g2 };
    duel::destroy(d, targets, REASON_EFFECT, None, None);
}
