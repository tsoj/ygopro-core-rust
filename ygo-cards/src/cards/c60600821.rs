//! Spring
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 60600821;
const COUNTER_SEASON: u16 = 0x214;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_SEASON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SEASON)
    card::enable_counter_permit(d, c, COUNTER_SEASON, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Select any number of your Main Monster Zones to make them unusuable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters you control gain 400 ATK for each Season Counter on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(function(e,c) return e:GetHandler():GetCounter(COUNTER_SEASON)*400 end)
    effect::set_value_fn(d, e2, effect_value(|d, e| {
        let h = effect::get_handler(d, e).nil();
        card::get_counter(d, h, COUNTER_SEASON) as i64 * 400
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 Field Spell from your Deck that you can place a Season Counter on face-up on your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.countertg(e,tp,...,chk)`
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local max_zones=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let max_zones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return max_zones>0 end
    if a.chk == Some(0) {
        return max_zones > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,max_zones,tp,COUNTER_SEASON)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, max_zones as u8, tp, COUNTER_SEASON as i32);
    true
}
/// `s.counterop(e,tp,...)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local max_zones=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let max_zones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if max_zones<=0 then return end
    if max_zones <= 0 {
        return;
    }
    // local ct=0
    let mut ct: i32 = 0;
    // local selected_zones=0
    let mut selected_zones: u32 = 0;
    // repeat
    loop {
        // ct=ct+1
        ct += 1;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISABLEZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISABLEZONE as u64);
        // local new_zone=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,selected_zones)
        let new_zone = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(selected_zones), false);
        // selected_zones=selected_zones|new_zone
        selected_zones |= new_zone;
        // until (ct>=max_zones or not Duel.SelectYesNo(tp,aux.Stringid(id,2)))
        if ct >= max_zones || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            break;
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:AddCounter(COUNTER_SEASON,ct)
    card::add_counter(d, c, COUNTER_SEASON, ct as u16, false);
    // --Those zones cannot be used while this card is on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetOperation(function() return selected_zones end)
    let zones = selected_zones;
    effect::set_operation(d, e1, Some(lua_fn(move |_d, _a: Args| zones as i64)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsCanAddCounter(COUNTER_SEASON,1,false,LOCATION_ONFIELD) and not c:IsForbidden()
    card::is_field_spell(d, c) && card::is_can_add_counter(d, c, COUNTER_SEASON, 1, false, LOCATION_ONFIELD) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,...,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,e:GetHandler():GetCounter(COUNTER_SEASON),tp,COUNTER_SEASON)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, COUNTER_SEASON);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, ct as u8, a.tp, COUNTER_SEASON as i32);
    true
}
/// `s.plop(e,tp,...)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(COUNTER_SEASON)
    let ct = card::get_counter(d, c, COUNTER_SEASON);
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // if Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) then
    if duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) {
        // --Cannot activate its effects this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3302)
        effect::set_description(d, e1, 3302);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // if ct>0 then
        if ct > 0 {
            // sc:AddCounter(COUNTER_SEASON,ct)
            card::add_counter(d, sc, COUNTER_SEASON, ct as u16, false);
        }
    }
}
