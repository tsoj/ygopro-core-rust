//! Tempest Magician
use super::prelude::*;

pub const ID: u32 = 63101919;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_SPELLCASTER),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, cc, sc, st, tp| card::is_race_ctx(d, cc, RACE_SPELLCASTER, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // synchro success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.addcc1)
    effect::set_condition(d, e1, Some(condition(addcc1)));
    // e1:SetTarget(s.addct1)
    effect::set_target(d, e1, Some(cost_target(addct1)));
    // e1:SetOperation(s.addc1)
    effect::set_operation(d, e1, Some(operation(addc1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.ctcost)
    effect::set_cost(d, e2, Some(cost_target(ctcost)));
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.damcost)
    effect::set_cost(d, e3, Some(cost_target(damcost)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.addcc1(e,...)`
fn addcc1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
/// `s.addct1(e,...,chk)`
fn addct1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,2,0,COUNTER_SPELL)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 2, 0, COUNTER_SPELL as i32);
    true
}
/// `s.addc1(e,...)`
fn addc1(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.ctcost(e,...,chk)`
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,63,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, 63, false, NO_EX).nil();
    let n = g.len();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[n as i64]);
    true
}
/// `s.cttg(e,...,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,e:GetLabel(),0,COUNTER_SPELL)
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, label as u8, 0, COUNTER_SPELL as i32);
    true
}
/// `s.ctop(e,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // for i=1,ct do
    for _ in 1..=ct {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
        // local g=Duel.SelectMatchingCard(tp,Card.IsCanAddCounter,tp,LOCATION_MZONE,0,1,1,nil,COUNTER_SPELL,1)
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // g:GetFirst():AddCounter(COUNTER_SPELL,1)
        let tc = group::get_first(d, &g).nil();
        card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_SPELL)>0
    card::get_counter(d, c, COUNTER_SPELL as u16) > 0
}
/// `s.damcost(e,...,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCounter(tp,1,1,COUNTER_SPELL)>0 end
    if a.chk == Some(0) {
        return duel::get_counter(d, a.tp, 1, 1, COUNTER_SPELL as u16) > 0;
    }
    // local g=Duel.GetMatchingGroup(s.damfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&damfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // local sum=0
    let mut sum: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local sct=tc:GetCounter(COUNTER_SPELL)
        let sct = card::get_counter(d, tc, COUNTER_SPELL as u16);
        // tc:RemoveCounter(tp,COUNTER_SPELL,sct,0)
        card::remove_counter(d, tc, a.tp, COUNTER_SPELL as u16, sct as u16, 0);
        // sum=sum+sct
        sum += sct as i64;
    }
    // e:SetLabel(sum)
    effect::set_label(d, a.e, &[sum]);
    true
}
/// `s.damtg(e,...,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(ct*500)
    duel::set_target_param(d, (ct * 500) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 500) as i32);
    true
}
/// `s.damop(e,...)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
