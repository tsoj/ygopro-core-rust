//! Autumn
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 101403071;
const CARD_SUMMER: u32 = 97254001;
const COUNTER_SEASON: u16 = 0x214;
const LOCATIONS_HAND_DECK_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SUMMER, ID], counter_place_list: &[COUNTER_SEASON as u32], ..ScriptInfo::DEFAULT };

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
    // This card's name becomes "Summer" while in the Field Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetValue(CARD_SUMMER)
    effect::set_value(d, e1, CARD_SUMMER as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn: You can place Season Counters on this card equal to the number of cards in your opponent's hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.countertg)
    effect::set_target(d, e2, Some(cost_target(countertg)));
    // e2:SetOperation(s.counterop)
    effect::set_operation(d, e2, Some(operation(counterop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // At the start of the Battle Phase: You can take 1 Field Spell from your hand, Deck, or GY that you can place a Season Counter on, except "Autumn", and place it face-up on your field, and if you do, place all Season Counters on this card on that card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_BATTLE_START)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_BATTLE_START);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // If a card(s) in the Field Zone would be destroyed by card effect, you can banish this card from your GY instead
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetTarget(s.reptg)
    effect::set_target(d, e4, Some(cost_target(reptg)));
    // e4:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e4, effect_card_fn(|d, e, c| {
        let _p = effect::get_handler_player(d, e);
        repfilter(d, c)
    }));
    // e4:SetOperation(function(e) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e4, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.countertg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    // local opp_hand_count=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let opp_hand_count = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if chk==0 then return opp_hand_count>0 end
    if a.chk == Some(0) {
        return opp_hand_count > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,opp_hand_count,tp,COUNTER_SEASON)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, opp_hand_count as u8, a.tp, COUNTER_SEASON as i32);
    true
}
/// `s.counterop(e,tp,eg,ep,ev,re,r,rp)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local opp_hand_count=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let opp_hand_count = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if c:IsRelateToEffect(e) and opp_hand_count>0 then
    if card::is_relate_to_effect(d, c, a.e) && opp_hand_count > 0 {
        // c:AddCounter(COUNTER_SEASON,opp_hand_count)
        card::add_counter(d, c, COUNTER_SEASON, opp_hand_count as u16, false);
    }
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsCanAddCounter(COUNTER_SEASON,1,false,LOCATION_ONFIELD) and not c:IsCode(id) and not c:IsForbidden()
    card::is_field_spell(d, c)
        && card::is_can_add_counter(d, c, COUNTER_SEASON, 1, false, LOCATION_ONFIELD)
        && !card::is_code(d, c, &[ID])
        && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATIONS_HAND_DECK_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATIONS_HAND_DECK_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,e:GetHandler():GetCounter(COUNTER_SEASON),tp,COUNTER_SEASON)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, COUNTER_SEASON);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, ct as u8, a.tp, COUNTER_SEASON as i32);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATIONS_HAND_DECK_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| plfilter(d, c))), a.tp, LOCATIONS_HAND_DECK_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(COUNTER_SEASON)
    let ct = card::get_counter(d, c, COUNTER_SEASON);
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // if Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) and ct>0 then
    if duel::move_to_field(d, sc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) && ct > 0 {
        // sc:AddCounter(COUNTER_SEASON,ct)
        card::add_counter(d, sc, COUNTER_SEASON, ct as u16, false);
    }
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_FZONE) and not c:IsReason(REASON_REPLACE)
    card::is_location(d, c, LOCATION_FZONE) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &a.eg.clone().nil(), &|d, x| repfilter(d, x), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
