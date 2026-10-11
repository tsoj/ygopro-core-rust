//! Granmarg the Mega Monarch
use super::prelude::*;

pub const ID: u32 = 15545291;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e2=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // local e2a=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2a = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.otfilter(c)`
fn otfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| desfilter(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DESTROY | CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
/// `s.dfilter(c,e)`
fn dfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFacedown() and c:IsRelateToEffect(e)
    card::is_facedown(d, c) && card::is_relate_to_effect(d, c, e)
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.dfilter,nil,e)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &tg, &|d, c| dfilter(d, c, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // if Duel.GetOperationInfo(0,CATEGORY_DRAW) then
    if duel::get_operation_info(d, 0, CATEGORY_DRAW).is_some() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH) then
    let lo = effect::get_label_object(d, e).as_effect().nil();
    if group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_EARTH), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    0
}
