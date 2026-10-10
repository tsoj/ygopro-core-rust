//! Proxy Dragon
use super::prelude::*;

pub const ID: u32 = 22862454;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.desreptg)
    effect::set_target(d, e1, Some(cost_target(desreptg)));
    // e1:SetValue(s.desrepval)
    effect::set_value_fn(d, e1, effect_card_fn(desrepval));
    // e1:SetOperation(s.desrepop)
    effect::set_operation(d, e1, Some(operation(desrepop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsMonster() and c:IsDestructable(e)
    //     and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    card::is_controler(d, c, tp)
        && card::is_monster(d, c)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetLinkedGroup()
    let g = card::get_linked_group(d, c);
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //     and g:IsExists(s.desfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX) && group::is_exists(d, &g, &|d, x| desfilter(d, x, a.e, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local sg=g:FilterSelect(tp,s.desfilter,1,1,nil,e,tp)
        let sg = group::filter_select(d, &g, tp, &|d, x| desfilter(d, x, a.e, tp), 1, 1, false, NO_EX).nil();
        // e:SetLabelObject(sg:GetFirst())
        let first = group::get_first(d, &sg).nil();
        effect::set_label_object(d, a.e, Value::Card(first));
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg, true);
        // sg:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    // else return false end
    } else {
        false
    }
}
/// `s.desrepval(e,c)`
fn desrepval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
