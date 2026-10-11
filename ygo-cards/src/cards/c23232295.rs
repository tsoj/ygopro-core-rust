//! BK Lead Yoke (Battlin' Boxer Lead Yoke)
use super::prelude::*;

pub const ID: u32 = 23232295;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_BATTLIN_BOXER),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_BATTLIN_BOXER as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attack up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_DETACH_MATERIAL)
    effect::set_code(d, e2, EVENT_DETACH_MATERIAL);
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_BATTLIN_BOXER)
    //     and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
}
// s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // if e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT) and Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if card::check_remove_overlay_card(d, h, a.tp, 1, REASON_EFFECT) && duel::select_effect_yes_no(d, a.tp, h, Some(96)) {
        // e:GetHandler():RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, h, a.tp, 1, 1, REASON_EFFECT);
        // local g=eg:Filter(s.repfilter,nil,tp)
        let g = group::filter(d, &eg, &|d, c| repfilter(d, c, a.tp), NO_EX);
        // if #g==1 then
        if g.len() == 1 {
            // e:SetLabelObject(g:GetFirst())
            let first = group::get_first(d, &g).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let first = group::get_first(d, &cg).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        }
        return true;
    }
    false
}
// s.repval(e,c)
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
// s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    true
}
// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(800)
        effect::set_value(d, e1, 800);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
