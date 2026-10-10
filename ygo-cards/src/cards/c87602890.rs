//! Zaborg the Mega Monarch
use super::prelude::*;

pub const ID: u32 = 87602890;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
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
fn otfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // e:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,1)
        let h = effect::get_handler(d, e).nil();
        let _ = card::register_flag_effect(d, h, ID, RESET_CHAIN, 0, 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // if Duel.Destroy(tc,REASON_EFFECT)==0 or (tc:GetPreviousAttributeOnField()&ATTRIBUTE_LIGHT)==0 then return end
    if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 || (card::get_previous_attribute_on_field(d, tc) & ATTRIBUTE_LIGHT) == 0 {
        return;
    }
    // local lv=tc:GetOriginalLevel()
    let mut lv = card::get_original_level(d, tc);
    // if tc:IsType(TYPE_XYZ) then lv=tc:GetOriginalRank() end
    if card::is_type(d, tc, TYPE_XYZ) {
        lv = card::get_original_rank(d, tc);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_EXTRA,0,lv,lv,nil)
    let mut g1 = duel::select_matching_card(d, tp, None, tp, LOCATION_EXTRA, 0, lv as u16, lv as u16, false, NO_EX).nil();
    // local tg=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let tg = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if e:GetHandler():GetFlagEffect(id)>0 then
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) > 0 {
        // Duel.ConfirmCards(tp,tg)
        duel::confirm_cards(d, tp, &tg);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g2=tg:Select(tp,lv,lv,nil)
        let g2 = group::select(d, &tg, tp, lv as u16, lv as u16, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
    } else {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local g2=tg:Select(1-tp,lv,lv,nil)
        let g2 = group::select(d, &tg, 1 - tp, lv as u16, lv as u16, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
    }
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(g1,REASON_EFFECT)
        duel::send_to_grave(d, g1, REASON_EFFECT, None, None);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_LIGHT) then
    let light = group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), 1, NO_EX);
    // e:GetLabelObject():SetLabel(1)
    // else e:GetLabelObject():SetLabel(0) end
    let obj = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, obj, &[if light { 1 } else { 0 }]);
    0
}
