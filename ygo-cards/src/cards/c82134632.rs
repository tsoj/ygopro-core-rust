//! Melffy Playhouse
use super::prelude::*;

pub const ID: u32 = 82134632;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return cards your opponent controls to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(
        d,
        e1,
        TIMING_DAMAGE_STEP as u32,
        Some((TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32),
    );
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    true
}
/// `s.beastxyzfilter(c)`
fn beastxyzfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_BEAST) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_hand(d, chkc, None);
    }
    // local xyzg=Duel.GetMatchingGroup(s.beastxyzfilter,tp,LOCATION_MZONE,0,nil)
    let xyzg = duel::get_matching_group(d, Some(&beastxyzfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==100
        let cost_chk = effect::get_label(d, a.e) == 100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk and Duel.CheckRemoveOverlayCard(tp,0,0,1,REASON_COST,xyzg)
        //     and Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil)
        return cost_chk
            && duel::check_remove_overlay_card(d, a.tp, 0, 0, 1, REASON_COST, Some(&xyzg))
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local xyz_max_ct=xyzg:GetSum(Card.GetOverlayCount)
    let xyz_max_ct = group::get_sum(d, &xyzg, &|d, c| card::get_overlay_count(d, c) as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVEXYZ)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVEXYZ as u64);
    // local detach_ct=Duel.RemoveOverlayCard(tp,0,0,1,xyz_max_ct,REASON_COST,xyzg)
    let detach_ct = duel::remove_overlay_card(d, a.tp, 0, 0, 1, xyz_max_ct as u16, REASON_COST, Some(&xyzg));
    // e:SetLabel(detach_ct)
    effect::set_label(d, a.e, &[detach_ct as i64]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,detach_ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, detach_ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then Duel.SendtoHand(tg,nil,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
    // local atkg=Duel.GetMatchingGroup(s.beastxyzfilter,tp,LOCATION_MZONE,0,nil)
    let atkg = duel::get_matching_group(d, Some(&beastxyzfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #atkg==0 then return end
    if atkg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=e:GetLabel()*500
    let atk = effect::get_label(d, a.e) * 500;
    // for tc in atkg:Iter() do
    for tc in group::iter(d, &atkg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
