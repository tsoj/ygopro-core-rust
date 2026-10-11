//! Crystal Tree
use super::prelude::*;

pub const ID: u32 = 47408488;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST], counter_place_list: &[0x6], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x6)
    card::enable_counter_permit(d, c, 0x6, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 Crystal Counter on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.ctcon1)
    effect::set_condition(d, e2, Some(condition(ctcon1)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetCondition(s.ctcon2)
    effect::set_condition(d, e3, Some(condition(ctcon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place "Crystal Beast" monsters to your Spell/Trap Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(s.plcost)
    effect::set_cost(d, e4, Some(cost_target(plcost)));
    // e4:SetTarget(s.pltg)
    effect::set_target(d, e4, Some(cost_target(pltg)));
    // e4:SetOperation(s.plop)
    effect::set_operation(d, e4, Some(operation(plop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_SZONE) and not c:IsPreviousLocation(LOCATION_SZONE) and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsFaceup()
    //   and c:IsOriginalType(TYPE_MONSTER)
    card::is_location(d, c, LOCATION_SZONE)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_faceup(d, c)
        && card::is_original_type(d, c, TYPE_MONSTER)
}
// function s.ctcon1(e,tp,eg,ep,ev,re,r,rp)
fn ctcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // if not eg:IsExists(s.ctfilter,1,nil) then return end
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &ctfilter, 1, NO_EX) {
        return false;
    }
    // if Duel.GetCurrentChain()>0 then
    if duel::get_current_chain(d, false) > 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
        // return false
        return false;
    }
    // return true
    true
}
// function s.ctcon2(e,tp,eg,ep,ev,re,r,rp)
fn ctcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetFlagEffect(id)>0 then
    if card::get_flag_effect(d, c, ID) > 0 {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
        // return true
        true
    } else {
        // return false
        false
    }
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x6,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x6, 1, false);
}
// function s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c);
    }
    // e:SetLabel(c:GetCounter(0x6))
    let n = card::get_counter(d, c, 0x6) as i64;
    effect::set_label(d, a.e, &[n]);
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
// function s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=e:GetHandler():GetCounter(0x6)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, 0x6);
    // if chk==0 then return ct>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>=ct-1
    //   and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,ct,nil) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 as i64 >= ct as i64 - 1
            && duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK, 0, ct as u32, NO_EX);
    }
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<ct then return end
    if (duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 as i64) < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        // Treat it as a Continuous Spell
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
