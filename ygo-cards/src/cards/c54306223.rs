//! Venom Swamp
use super::prelude::*;

pub const ID: u32 = 54306223;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VENOM],
    counter_place_list: &[COUNTER_VENOM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.acop)
    effect::set_operation(d, e2, Some(operation(acop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(COUNTER_VENOM)*-500
    card::get_counter(d, c, COUNTER_VENOM as u16) as i64 * -500
}
// function s.acop(e,tp,eg,ep,ev,re,r,rp)
fn acop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local tg=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let tg = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // local tc=tg:GetFirst()
    let _ = group::get_first(d, &tg);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // if tc:IsCanAddCounter(COUNTER_VENOM,1) and not tc:IsSetCard(SET_VENOM) then
        if card::is_can_add_counter(d, tc, COUNTER_VENOM as u16, 1, false, 0) && !card::is_set_card(d, tc, &[SET_VENOM as u16]) {
            // local atk=tc:GetAttack()
            let atk = card::get_attack(d, tc);
            // tc:AddCounter(COUNTER_VENOM,1)
            card::add_counter(d, tc, COUNTER_VENOM as u16, 1, false);
            // if atk>0 and tc:GetAttack()==0 then
            if atk > 0 && card::get_attack(d, tc) == 0 {
                // g:AddCard(tc)
                group::add_card(&mut g, &CardSet::from_card(tc));
            }
        }
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.RaiseEvent(g,EVENT_CUSTOM+id,e,0,0,0,0)
        duel::raise_event(d, g, EVENT_CUSTOM + ID, Some(a.e), 0, 0, 0, 0);
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(eg)
    let eg = a.eg.clone().nil();
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=eg:Filter(Card.IsRelateToEffect,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
