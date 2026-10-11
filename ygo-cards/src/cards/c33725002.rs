//! V Salamander
use super::prelude::*;

pub const ID: u32 = 33725002;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[66970002], listed_series: &[SET_UTOPIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Utopia" monster from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip this card to 1 "Number C39: Utopia Ray V" you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all monsters your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetCost(s.descost)
    effect::set_cost(d, e3, Some(cost_target(descost)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():GetEquipTarget() end` (of e3)
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_UTOPIA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_UTOPIA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsCode(66970002) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc) && card::is_code(d, chkc, &[66970002]);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,66970002),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[66970002])), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,66970002),tp,LOCATION_MZONE,0,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[66970002])), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() or c:IsControler(1-tp) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) || card::is_controler(d, c, 1 - a.tp) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsControler(1-tp) or tc:IsFacedown() or not tc:IsRelateToEffect(e)
    //     or not tc:IsCode(66970002) then
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        // Duel.SendtoGrave(c,REASON_RULE)
        duel::send_to_grave(d, c, REASON_RULE, None, None);
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_controler(d, tc, 1 - a.tp)
        || card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || !card::is_code(d, tc, &[66970002])
    {
        duel::send_to_grave(d, c, REASON_RULE, None, None);
        return;
    }
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, a.tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetValue(function(e,c) return c==e:GetLabelObject() end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| effect::get_label_object(d, e).as_card() == Some(c)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // if chk==0 then return ec:CheckRemoveOverlayCard(tp,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, ec, a.tp, 1, REASON_COST);
    }
    // ec:RemoveOverlayCard(tp,1,1,REASON_COST)
    card::remove_overlay_card(d, ec, a.tp, 1, 1, REASON_COST);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 and ec:IsNegatableMonster() end
    if a.chk == Some(0) {
        return !g.is_empty() && card::is_negatable_monster(d, ec.nil());
    }
    // Duel.SetTargetCard(ec)
    let ec = ec.nil();
    duel::set_target_card(d, ec);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,ec,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(ec)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 1000) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local ec=Duel.GetFirstTarget()
    let ec = duel::first_target(d).nil();
    // if not (ec:IsRelateToEffect(e) and ec:IsNegatableMonster() and not ec:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, ec, a.e) && card::is_negatable_monster(d, ec) && !card::is_immune_to_effect(d, ec, a.e)) {
        return;
    }
    // ec:NegateEffects(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    card::negate_effects(d, ec, h, None, false, None);
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(1-tp,ct*1000,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, ct as i64 * 1000, REASON_EFFECT, false, None);
    }
}
