//! Morphtronic Cameran
use super::prelude::*;

pub const ID: u32 = 28124263;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_MORPHTRONIC}
    listed_series: &[SET_MORPHTRONIC],
    // s.listed_names={id}
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD_P);
    // e1:SetOperation(s.check)
    effect::set_operation(d, e1, Some(operation(check)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.cona)
    effect::set_condition(d, e2, Some(condition(cona)));
    // e2:SetTarget(s.tga)
    effect::set_target(d, e2, Some(cost_target(tga)));
    // e2:SetOperation(s.opa)
    effect::set_operation(d, e2, Some(operation(opa)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // untargetable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCondition(s.cond)
    effect::set_condition(d, e3, Some(lua_fn(cond)));
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MORPHTRONIC))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]))));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.check(e,tp,eg,ep,ev,re,r,rp)
fn check(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsDisabled() and c:IsAttackPos() then e:SetLabel(1)
    if !card::is_disabled(d, c) && card::is_attack_pos(d, c) {
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
// function s.cona(e,tp,eg,ep,ev,re,r,rp)
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()==1
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) == 1
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_MORPHTRONIC) and not c:IsCode(id)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.tga(e,tp,eg,ep,ev,re,r,rp,chk)
fn tga(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.opa(e,tp,eg,ep,ev,re,r,rp)
fn opa(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.e, tp))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
// function s.cond(e)
fn cond(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, h)
}
