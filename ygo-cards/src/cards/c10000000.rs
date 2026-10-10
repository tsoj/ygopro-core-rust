//! Obelisk the Tormentor
use super::prelude::*;

pub const ID: u32 = 10000000;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    let _e2 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // summon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_CANNOT_DISABLE_SUMMON);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    card::register_effect(d, c, e3, false);
    // summon success
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e4, Some(operation(sumsuc)));
    card::register_effect(d, c, e4, false);
    // cannot be target
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_value(d, e5, 1);
    card::register_effect(d, c, e5, false);
    // to grave
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_category(d, e6, CATEGORY_TOGRAVE);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    effect::set_condition(d, e6, Some(condition(tgcon)));
    effect::set_target(d, e6, Some(cost_target(tgtg)));
    effect::set_operation(d, e6, Some(operation(tgop)));
    card::register_effect(d, c, e6, false);
    // destroy
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 1));
    effect::set_category(d, e7, CATEGORY_DESTROY);
    effect::set_type(d, e7, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_cost(d, e7, Some(cost_target(descost)));
    effect::set_target(d, e7, Some(cost_target(destg)));
    effect::set_operation(d, e7, Some(operation(desop)));
    card::register_effect(d, c, e7, false);
}
/// `s.sumsuc(e,...)`
fn sumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(aux.FALSE)
    duel::set_chain_limit_till_chain_end(d, aux_false());
}
/// `s.tgcon(e,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSpecialSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_special_summoned(d, h)
}
/// `s.tgtg(e,...,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tgop(e,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then Duel.SendtoGrave(c,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.descost(e,...,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetAttackAnnouncedCount()==0 and Duel.CheckReleaseGroupCost(tp,nil,2,false,aux.ReleaseCheckTarget,nil,dg) end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, h) == 0
            && aux::check_release_group_cost(
                d,
                tp,
                None,
                2,
                2,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
                NO_EX,
            );
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, h, e1, false);
    // local g=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,aux.ReleaseCheckTarget,nil,dg)
    let g = aux::select_release_group_cost(
        d,
        tp,
        None,
        2,
        2,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
