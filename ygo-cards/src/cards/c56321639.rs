//! Spellbook Star Hall
use super::prelude::*;

pub const ID: u32 = 56321639;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], counter_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // add counters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetOperation(aux.chainreg)
    effect::set_operation(d, e1, Some(aux::chainreg()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_SPELLCASTER))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_SPELLCASTER))));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // registration before leaving
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // add a Spellcaster to hand
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e5:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // e5:SetCondition(s.thcon)
    effect::set_condition(d, e5, Some(condition(thcon)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    let Some(re) = a.re else { return false };
    // local c=re:GetHandler()
    let c = effect::get_handler(d, re);
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and c:IsSetCard(SET_SPELLBOOK) and e:GetHandler():GetFlagEffect(1)>0
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_effect(d, re)
        && card::is_set_card(d, c.nil(), &[SET_SPELLBOOK as u16])
        && card::get_flag_effect(d, effect::get_handler(d, a.e).nil(), 1) > 0
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(COUNTER_SPELL,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetCounter(COUNTER_SPELL)*100
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, COUNTER_SPELL as u16) as i64 * 100
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(COUNTER_SPELL)
    let ct = card::get_counter(d, c, COUNTER_SPELL as u16);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabelObject():GetLabel()
    let e4 = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, e4);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // return ct>0 and c:IsReason(REASON_DESTROY)
    ct > 0 && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.filter(c,level)`
fn filter(d: &mut Duel, c: CardId, level: i64) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_SPELLCASTER) and c:IsLevelBelow(level) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_level_below(d, c, level as u32) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e:GetLabel()) end
    if a.chk == Some(0) {
        let lv = effect::get_label(d, a.e);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let lv = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
