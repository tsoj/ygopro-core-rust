//! War Rock Ordeal
use super::prelude::*;

pub const ID: u32 = 71331215;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_WAR_ROCK],
    counter_place_list: &[0x205],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // c:EnableCounterPermit(0x205)
    card::enable_counter_permit(d, c, 0x205, None, None);
    // place counters on activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove counter + draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_COUNTER+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_COUNTER + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.dcon)
    effect::set_condition(d, e2, Some(condition(dcon)));
    // e2:SetTarget(s.dtg)
    effect::set_target(d, e2, Some(cost_target(dtg)));
    // e2:SetOperation(s.dop)
    effect::set_operation(d, e2, Some(operation(dop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to GY when last counter is removed
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_REMOVE_COUNTER+0x205)
    effect::set_code(d, e3, EVENT_REMOVE_COUNTER + 0x205);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(condition(tgcon)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // e:GetHandler():AddCounter(0x205,3)
    card::add_counter(d, h, 0x205, 3, false);
}
/// `s.dcon(e,tp,eg,ep,ev,re,r,rp)`
fn dcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg()).nil();
    // local rc=tc:GetReasonCard()
    let rc = card::get_reason_card(d, tc);
    // return #eg==1 and rc:IsControler(tp) and tc:IsMonster() and tc:IsReason(REASON_BATTLE) and tc:IsLocation(LOCATION_GRAVE) and rc:IsSetCard(SET_WAR_ROCK)
    if a.eg().len() != 1 {
        return false;
    }
    let rc = rc.nil();
    card::is_controler(d, rc, a.tp)
        && card::is_monster(d, tc)
        && card::is_reason(d, tc, REASON_BATTLE)
        && card::is_location(d, tc, LOCATION_GRAVE)
        && card::is_set_card(d, rc, &[SET_WAR_ROCK as u16])
}
/// `s.dtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and e:GetHandler():IsCanRemoveCounter(tp,0x205,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && card::is_can_remove_counter(d, h, a.tp, 0x205, 1, REASON_EFFECT);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,e:GetHandler(),1,tp,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(h)), 1, a.tp, LOCATION_SZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.dop(e,tp,eg,ep,ev,re,r,rp)`
fn dop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(0x205)
    let ct = card::get_counter(d, c, 0x205);
    // e:GetHandler():RemoveCounter(tp,0x205,1,REASON_EFFECT)
    card::remove_counter(d, c, a.tp, 0x205, 1, REASON_EFFECT);
    // if c:GetCounter(0x205)<ct then
    if card::get_counter(d, c, 0x205) < ct {
        // Duel.RaiseEvent(c,EVENT_REMOVE_COUNTER+0x205,e,REASON_EFFECT,tp,tp,1)
        duel::raise_event(d, c, EVENT_REMOVE_COUNTER + 0x205, Some(a.e), REASON_EFFECT, a.tp, a.tp, 1);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x205)==0
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x205) == 0
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT, None, None);
}
