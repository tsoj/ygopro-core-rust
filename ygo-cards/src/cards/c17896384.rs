//! Arcane Barrier
use super::prelude::*;

pub const ID: u32 = 17896384;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,4)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 4);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCost(s.drcost)
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP) and (c:GetPreviousRaceOnField()&RACE_SPELLCASTER)~=0
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && (card::get_previous_race_on_field(d, c) & RACE_SPELLCASTER) != 0
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctfilter,1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| ctfilter(d, c), 1, NO_EX)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(COUNTER_SPELL,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h) && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // e:SetLabel(e:GetHandler():GetCounter(COUNTER_SPELL))
    let counters = card::get_counter(d, h, COUNTER_SPELL as u16);
    effect::set_label(d, a.e, &[counters as i64]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetCounter(COUNTER_SPELL)>0 end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_counter(d, h, COUNTER_SPELL as u16) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, effect::get_label(d, a.e) as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
