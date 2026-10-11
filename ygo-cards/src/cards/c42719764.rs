//! Summon Shock
use super::prelude::*;

pub const ID: u32 = 42719764;
/// `local COUNTER_SUMMON=0x148`
const COUNTER_SUMMON: u16 = 0x148;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SUMMON as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SUMMON)
    card::enable_counter_permit(d, c, COUNTER_SUMMON, None, None);
    // c:SetCounterLimit(COUNTER_SUMMON,4)
    card::set_counter_limit(d, c, COUNTER_SUMMON, 4);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e0, 0, Some(TIMING_STANDBY_PHASE));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Each time a monster(s) is Normal or Special Summoned, place 1 Summon Counter on this card (max. 4)
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1a, LOCATION_SZONE);
    // e1a:SetOperation(s.counterop)
    effect::set_operation(d, e1a, Some(operation(counterop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // If the 4th Summon Counter is placed on this card: Remove all Summon Counters from this card, and if you do, send all monsters on the field to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.counterop(e,tp,eg,ep,ev,re,r,rp)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:AddCounter(COUNTER_SUMMON,1) and c:GetCounter(COUNTER_SUMMON)==4 then
    if card::add_counter(d, c, COUNTER_SUMMON, 1, false) && card::get_counter(d, c, COUNTER_SUMMON) == 4 {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,0,0,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, 0, 0, tp, 0);
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // if c:IsRelateToEffect(e) and c:RemoveCounter(tp,COUNTER_SUMMON,c:GetCounter(COUNTER_SUMMON),REASON_EFFECT)
    //     and c:GetCounter(COUNTER_SUMMON)==0 and #g>0 then
    if card::is_relate_to_effect(d, c, a.e) {
        let n = card::get_counter(d, c, COUNTER_SUMMON) as u16;
        if card::remove_counter(d, c, tp, COUNTER_SUMMON, n, REASON_EFFECT) && card::get_counter(d, c, COUNTER_SUMMON) == 0 && !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
