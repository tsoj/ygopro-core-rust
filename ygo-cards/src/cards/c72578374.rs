//! Khaos Starsource Dragon
use super::prelude::*;

pub const ID: u32 = 72578374;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 LIGHT or DARK Dragon monster + 1 Dragon monster
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // Destroy cards your opponent controls up to the number of your LIGHT and DARK monsters used as this card's Fusion Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and e:GetLabel()>0 end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h) && effect::get_label(d, a.e) > 0
    })));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register the materials used
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1a, EFFECT_MATERIAL_CHECK);
    // e1a:SetValue(s.valcheck)
    effect::set_value_fn(d, e1a, lua_fn(valcheck));
    // e1a:SetLabelObject(e1)
    effect::set_label_object(d, e1a, Value::Effect(e1));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Make 1 Level 4 LIGHT or DARK Dragon monster you control become Level 8
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,fc,st,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK,fc,st,tp) and c:IsRace(RACE_DRAGON,fc,st,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, Some(fc), st, Some(tp))
        && card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,e:GetLabel(),nil)
    let max = effect::get_label(d, a.e) as u16;
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, max, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.valcheckfilter(c,fc,tp)`
fn valcheckfilter(d: &mut Duel, c: CardId, fc: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK,fc,SUMMON_TYPE_FUSION,tp) and c:IsControler(tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, Some(fc), SUMMON_TYPE_FUSION as u64, Some(tp))
        && card::is_controler(d, c, tp)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local ct=c:GetMaterial():FilterCount(s.valcheckfilter,nil,c,e:GetHandlerPlayer())
    let tp = effect::get_handler_player(d, e);
    let mat = card::get_material(d, c);
    let ct = group::filter_count(d, &mat, &|d, x| valcheckfilter(d, x, c, tp), NO_EX);
    // e:GetLabelObject():SetLabel(ct)
    let le = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, le, &[ct as i64]);
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsRace(RACE_DRAGON) and c:IsFaceup()
    card::is_level(d, c, &[4]) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| lvfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| lvfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,g,1,tp,8)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, g, 1, a.tp, 8);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:HasLevel() and not tc:IsLevel(8) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::has_level(d, tc) && !card::is_level(d, tc, &[8]) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(8)
        effect::set_value(d, e1, 8);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
