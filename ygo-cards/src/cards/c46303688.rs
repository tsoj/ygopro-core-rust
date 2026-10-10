//! Roulette Barrel
use super::prelude::*;

pub const ID: u32 = 46303688;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DICE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    true
}
/// `s.dfilter(c,lv)`
fn dfilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsFaceup() and c:GetLevel()==lv
    card::is_faceup(d, c) && card::get_level(d, c) as i64 == lv
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local d1,d2=Duel.TossDice(tp,2)
    let r = duel::toss_dice(d, a.tp, 2, 0);
    let mut d1 = r[0];
    let mut d2 = r[1];
    // local sel=d1
    let mut sel = d1;
    // if d1>d2 then d1,d2=d2,d1 end
    if d1 > d2 {
        std::mem::swap(&mut d1, &mut d2);
    }
    // if d1~=d2 then
    if d1 != d2 {
        // sel=Duel.AnnounceNumber(tp,d1,d2)
        let (n, _) = duel::announce_number(d, a.tp, &[d1 as u64, d2 as u64]).nil();
        sel = n as i32;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.dfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,sel)
    let dg = duel::select_matching_card(d, a.tp, Some(&|d, c| dfilter(d, c, sel as i64)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
