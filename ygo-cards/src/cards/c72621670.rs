//! Double Magical Arm Bind
use super::prelude::*;

pub const ID: u32 = 72621670;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `function s.chk(sg,tp,exg,dg) return dg:IsExists(aux.TRUE,2,sg) end`
fn chk(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> (bool, bool) {
    (group::is_exists(d, dg, &|_, _| true, 2, sg.into()), false)
}
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsAbleToChangeControler() and (not e or c:IsCanBeEffectTarget(e))
    if !card::is_faceup(d, c) || !card::is_able_to_change_controler(d, c) {
        return false;
    }
    match e {
        None => true,
        Some(_) => card::is_can_be_effect_target(d, c, e),
    }
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e)
    let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(a.e))), tp, 0, LOCATION_MZONE, NO_EX);
    let check = |d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| chk(d, sg, &dg);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,false,s.chk,nil,dg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, None, 2, 2, false, Some(&check), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,s.chk,nil,dg)
    let g = duel::select_release_group_cost(d, tp, None, 2, 2, false, Some(&check), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,1-tp,LOCATION_REASON_CONTROL)>=0
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,2,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(1 - tp), Some(LOCATION_REASON_CONTROL), None).0 >= 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, None)), tp, 0, LOCATION_MZONE, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,2,2,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, None)), tp, 0, LOCATION_MZONE, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 2, 0, 0);
    true
}
fn tfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRelateToEffect(e) and c:IsFaceup()
    card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.tfilter,nil,e)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let targets = info.first().and_then(|v| v.as_group().cloned()).nil();
    let e = a.e;
    let g = group::filter(d, &targets, &|d, c| tfilter(d, c, e), NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local rct=1
    let mut rct: u8 = 1;
    // if Duel.IsTurnPlayer(1-tp) then rct=2 end
    if duel::is_turn_player(d, 1 - tp) {
        rct = 2;
    }
    // Duel.GetControl(g,tp,PHASE_END,rct)
    duel::get_control(d, g, tp, PHASE_END as u16, rct, None, None);
}
