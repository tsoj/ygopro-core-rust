//! Dogoran, the Mad Flame Kaiju
use super::prelude::*;

pub const ID: u32 = 93332803;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_KAIJU as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1,e2=aux.AddKaijuProcedure(c)
    aux::add_kaiju_procedure(d, c);
    // Destroy all of opponent's monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.descost)
    effect::set_cost(d, e3, Some(cost_target(descost)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_KAIJU,3,REASON_COST)
    //     and e:GetHandler():GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_can_remove_counter(d, tp, 1, 1, COUNTER_KAIJU as u16, 3, REASON_COST) && card::get_attack_announced_count(d, h) == 0;
    }
    // Duel.RemoveCounter(tp,1,1,COUNTER_KAIJU,3,REASON_COST)
    duel::remove_counter(d, tp, 1, 1, COUNTER_KAIJU as u16, 3, REASON_COST);
    // Cannot attack this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
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
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
