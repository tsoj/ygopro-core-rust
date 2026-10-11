//! The Agent of Judgment - Saturn
use super::prelude::*;

pub const ID: u32 = 91345518;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict damage to the opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.damcost)
    effect::set_cost(d, e1, Some(cost_target(damcost)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetCurrentPhase()~=PHASE_MAIN2 and e:GetHandler():IsReleasable() end
    if a.chk == Some(0) {
        return duel::get_current_phase(d) != PHASE_MAIN2 && card::is_releasable(d, h, None, None);
    }
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, h, REASON_COST, None);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    true
}
/// `s.envfilter(c)`
fn envfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_SANCTUARY_SKY)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SANCTUARY_SKY])
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLP(tp)>Duel.GetLP(1-tp)
    //     and (Duel.IsExistingMatchingCard(s.envfilter,tp,LOCATION_ONFIELD,0,1,nil) or Duel.IsEnvironment(CARD_SANCTUARY_SKY,tp)) end
    if a.chk == Some(0) {
        return duel::get_lp(d, tp) > duel::get_lp(d, 1 - tp)
            && (duel::is_existing_matching_card(d, Some(&envfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX) || duel::is_environment(d, CARD_SANCTUARY_SKY, Some(tp), None));
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not (Duel.IsExistingMatchingCard(s.envfilter,tp,LOCATION_ONFIELD,0,1,nil) or Duel.IsEnvironment(CARD_SANCTUARY_SKY,tp)) then return end
    if !(duel::is_existing_matching_card(d, Some(&envfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX) || duel::is_environment(d, CARD_SANCTUARY_SKY, Some(tp), None)) {
        return;
    }
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local val=Duel.GetLP(1-p)-Duel.GetLP(p)
    let val = duel::get_lp(d, 1 - p) - duel::get_lp(d, p);
    // if val>0 then
    if val > 0 {
        // Duel.Damage(p,val,REASON_EFFECT)
        duel::damage(d, p, val as i64, REASON_EFFECT, false, None);
    }
}
