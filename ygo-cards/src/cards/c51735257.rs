//! Number 50: Blackship of Corn
use super::prelude::*;

pub const ID: u32 = 51735257;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(50), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Send 1 monster your opponent controls to the GY, and if you do, inflict 1000 damage to them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),s.tgcost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(tgcost)])));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // This card cannot attack during the turn you activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    true
}
/// `s.tgfilter(c,atk)`: `return c:IsAttackBelow(atk) and c:IsFaceup() and c:IsAbleToGrave()`
fn tgfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_attack_below(d, c, atk) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=e:GetHandler():GetAttack()
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h);
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,atk)
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_MZONE,1,nil,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_GRAVE) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_GRAVE)
    {
        // Duel.Damage(1-tp,1000,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, false, None);
    }
}
