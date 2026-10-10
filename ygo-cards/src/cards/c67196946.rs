//! Star Blast
use super::prelude::*;

pub const ID: u32 = 67196946;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // for _,te in ipairs({Duel.GetPlayerEffect(tp,EFFECT_LPCOST_CHANGE)}) do
    for te in duel::get_player_effect(d, a.tp, EFFECT_LPCOST_CHANGE) {
        // local val=te:GetValue()
        let Value::Fn(f) = effect::get_value(d, te) else {
            crate::lua_error("attempt to call a number value");
        };
        // if val(te,e,tp,500)~=500 then return false end
        if f.call(d, &[Value::Effect(te), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(500)]).int() != 500 {
            return false;
        }
    }
    // return true
    true
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:GetLevel()>lv
    card::get_level(d, c) as i64 > lv
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return true end
    true
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckLPCost(tp,500) and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,1)
        return duel::check_lp_cost(d, a.tp, 500) && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, 1)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp) as i64;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,1)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, 1)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // local tg=g:GetMaxGroup(Card.GetLevel)
    let tg = group::get_max_group(d, &g, &|d, c| card::get_level(d, c) as i64).nil().0;
    // local maxlv=tg:GetFirst():GetLevel()
    let first = group::get_first(d, &tg).nil();
    let maxlv = card::get_level(d, first) as i64;
    // local t={}
    let mut t: Vec<u64> = Vec::new();
    // local l=1
    let mut l: i64 = 1;
    // while l<maxlv and l*500<=lp do
    while l < maxlv && l * 500 <= lp {
        // t[l]=l*500
        t.push((l * 500) as u64);
        // l=l+1
        l += 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local announce=Duel.AnnounceNumber(tp,table.unpack(t))
    let announce = duel::announce_number(d, a.tp, &t).nil().0;
    // Duel.PayLPCost(tp,announce)
    duel::pay_lp_cost(d, a.tp, announce as u32);
    // Duel.SetTargetParam(announce/500)
    duel::set_target_param(d, (announce / 500) as u32);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    let ct = info[0].int();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,ct)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, ct)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // if tc:IsLocation(LOCATION_MZONE) then
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, &sg, true);
        }
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, &sg);
        // if tc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if card::is_location(d, tc, LOCATION_HAND) {
            duel::shuffle_hand(d, a.tp);
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        // e1:SetValue(-ct)
        effect::set_value(d, e1, -ct);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
