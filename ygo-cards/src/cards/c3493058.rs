//! Dicephoon
use super::prelude::*;

pub const ID: u32 = 3493058;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_DICE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    // if dc==1 or dc==6 then
    if dc == 1 || dc == 6 {
        // Duel.Damage(tp,1000,REASON_EFFECT)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
    } else if dc == 5 {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        // if #g<2 then return end
        if g.len() < 2 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=g:Select(tp,2,2,nil)
        let dg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into()).nil();
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
