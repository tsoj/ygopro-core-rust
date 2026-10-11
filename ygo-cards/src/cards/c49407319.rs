//! Star Mine
use super::prelude::*;

pub const ID: u32 = 49407319;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // SS limit
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // damage1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCondition(s.damcon1)
    effect::set_condition(d, e1, Some(condition(damcon1)));
    // e1:SetTarget(s.damtg1)
    effect::set_target(d, e1, Some(cost_target(damtg1)));
    // e1:SetOperation(s.damop1)
    effect::set_operation(d, e1, Some(operation(damop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage2
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    // (sets the description of e1 after it is registered, as the Lua does)
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCondition(s.damcon2)
    effect::set_condition(d, e2, Some(condition(damcon2)));
    // e2:SetTarget(s.damtg2)
    effect::set_target(d, e2, Some(cost_target(damtg2)));
    // e2:SetOperation(s.damop2)
    effect::set_operation(d, e2, Some(operation(damop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,tp,rp)`
fn filter(d: &mut Duel, c: CardId, tp: u8, rp: u8) -> bool {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // if c:IsReason(REASON_BATTLE) and ac and ac~=c then
    if card::is_reason(d, c, REASON_BATTLE) && ac.is_some() && ac != Some(c) {
        let ac = ac.nil();
        // if ac:IsLocation(LOCATION_MZONE) then return ac:IsControler(1-tp) else return ac:IsPreviousControler(1-tp) end
        if card::is_location(d, ac, LOCATION_MZONE) {
            card::is_controler(d, ac, 1 - tp)
        } else {
            card::is_previous_controler(d, ac, 1 - tp)
        }
    // elseif c:IsReason(REASON_EFFECT) and rp~=tp then
    } else if card::is_reason(d, c, REASON_EFFECT) && rp != tp {
        // return true
        true
    // end
    // return false
    } else {
        false
    }
}
/// `s.damcon1(e,tp,eg,ep,ev,re,r,rp)`
fn damcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return s.filter(e:GetHandler(),tp,rp) and e:GetHandler():IsPreviousControler(tp)
    let h = effect::get_handler(d, a.e).nil();
    filter(d, h, a.tp, a.rp) && card::is_previous_controler(d, h, a.tp)
}
/// `s.damtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,2000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 2000);
    true
}
/// `s.damop1(e,tp,eg,ep,ev,re,r,rp)`
fn damop1(d: &mut Duel, a: &EffArgs) {
    // local val=Duel.Damage(tp,2000,REASON_EFFECT)
    let val = duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
    // if val>0 and Duel.GetLP(tp)>0 then
    if val > 0 && duel::get_lp(d, a.tp) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,2000,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
    }
}
/// `s.cfilter(c,tp,seq)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, seq: i64) -> bool {
    // if not c:IsPreviousControler(tp) or not c:IsPreviousLocation(LOCATION_MZONE) then return false end
    if !card::is_previous_controler(d, c, tp) || !card::is_previous_location(d, c, LOCATION_MZONE) {
        return false;
    }
    // return (seq-1>=0 and c:GetPreviousSequence()==seq-1) or (seq+1<=4 and c:GetPreviousSequence()==seq+1)
    let ps = card::get_previous_sequence(d, c) as i64;
    (seq - 1 >= 0 && ps == seq - 1) || (seq + 1 <= 4 && ps == seq + 1)
}
/// `s.damcon2(e,tp,eg,ep,ev,re,r,rp)`
fn damcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:Filter(s.filter,nil,tp,rp):IsExists(s.cfilter,1,nil,tp,e:GetHandler():GetSequence())
    let eg = a.eg.clone().nil();
    let (tp, rp) = (a.tp, a.rp);
    let fg = group::filter(d, &eg, &|d, c| filter(d, c, tp, rp), NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    let seq = card::get_sequence(d, h) as i64;
    group::is_exists(d, &fg, &|d, c| cfilter(d, c, tp, seq), 1, NO_EX)
}
/// `s.damtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Group.FromCards(e:GetHandler()),1,tp,LOCATION_MZONE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,2000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 2000);
    true
}
/// `s.damop2(e,tp,eg,ep,ev,re,r,rp)`
fn damop2(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and Duel.Destroy(e:GetHandler(),REASON_EFFECT)>0 then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && duel::destroy(d, h, REASON_EFFECT, None, None) > 0 {
        // local val=Duel.Damage(tp,2000,REASON_EFFECT)
        let val = duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
        // if val>0 and Duel.GetLP(tp)>0 then
        if val > 0 && duel::get_lp(d, a.tp) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,2000,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
        }
    }
}
