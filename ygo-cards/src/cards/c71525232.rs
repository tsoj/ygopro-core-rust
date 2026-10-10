//! Gandora-X the Dragon of Demolition
use super::prelude::*;

pub const ID: u32 = 71525232;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy as many other monsters on the field as possible
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_summon_location(d, c, LOCATION_HAND)
    })));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Halve your LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e3:SetOperation(function(e,tp) Duel.SetLP(tp,math.ceil(Duel.GetLP(tp)/2)) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        let lp = duel::get_lp(d, a.tp);
        duel::set_lp(d, a.tp, ((lp as f64) / 2.0).ceil() as i32);
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,c)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // local _,atk=g:GetMaxGroup(Card.GetTextAttack)
    let atk = group::get_max_group(d, &g, &|d, x| card::get_text_attack(d, x) as i64).map(|x| x.1);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk.unwrap_or(0) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, e) { Some(c) } else { None };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,exc)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, exc.into());
    // if #g==0 or Duel.Destroy(g,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::destroy(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // local _,atk=og:GetMaxGroup(Card.GetTextAttack)
    let atk = group::get_max_group(d, &og, &|d, x| card::get_text_attack(d, x) as i64).nil().1;
    // if atk<=0 then return end
    if atk <= 0 {
        return;
    }
    // local dam=Duel.Damage(1-tp,atk,REASON_EFFECT)
    let dam = duel::damage(d, 1 - tp, atk, REASON_EFFECT, false, None);
    // if dam>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if dam > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
        // This card's ATK becomes equal to the damage inflicted to your opponent
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(dam)
        effect::set_value(d, e1, dam as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
