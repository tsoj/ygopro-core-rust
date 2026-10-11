//! Gouki The Great Ogre
use super::prelude::*;

pub const ID: u32 = 78437364;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GOUKI as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atkdown
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| atkval(d, c)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // e2:SetOperation(s.desrepop)
    effect::set_operation(d, e2, Some(operation(desrepop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, c: CardId) -> i64 {
    // local val=math.max(c:GetBaseDefense(),0)
    let val = card::get_base_defense(d, c).max(0) as i64;
    // return val*-1
    val * -1
}
/// `s.repfilter(c,e,tp)`
fn repfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=c:GetLinkedGroup()
        let g = card::get_linked_group(d, c);
        // return not c:IsReason(REASON_REPLACE) and g:IsExists(s.repfilter,1,nil,e,tp)
        return !card::is_reason(d, c, REASON_REPLACE) && group::is_exists(d, &g, &|d, cc| repfilter(d, cc, e, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // local g=c:GetLinkedGroup()
        let g = card::get_linked_group(d, c);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local sg=g:FilterSelect(tp,s.repfilter,1,1,nil,e,tp)
        let sg = group::filter_select(d, &g, tp, &|d, cc| repfilter(d, cc, e, tp), 1, 1, false, NO_EX).nil();
        // e:SetLabelObject(sg:GetFirst())
        let first = group::get_first(d, &sg).nil();
        effect::set_label_object(d, e, Value::Card(first));
        // sg:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT | REASON_REPLACE, None, None);
}
