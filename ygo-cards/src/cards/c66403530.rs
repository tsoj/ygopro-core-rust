//! Topologic Zeroboros
use super::prelude::*;

pub const ID: u32 = 66403530;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Disable zones
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e1, EFFECT_FORCE_MZONE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.znval)
    effect::set_value_fn(d, e1, lua_fn(znval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish all cards on field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special summon itself
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_REMOVE)
    effect::set_code(d, e4, EVENT_REMOVE);
    // e4:SetOperation(s.spreg)
    effect::set_operation(d, e4, Some(operation(spreg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e5, LOCATION_REMOVED);
    // e5:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e5, EVENT_PHASE | PHASE_STANDBY);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.znval(e)`
fn znval(d: &mut Duel, a: Args) -> i64 {
    // return ~(e:GetHandler():GetLinkedZone()&ZONES_EMZ)
    let c = effect::get_handler(d, a.e(0)).nil();
    !((card::get_linked_zone(d, c, None) & ZONES_EMZ) as i64)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_REMOVED,LOCATION_REMOVED)*200
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_REMOVED, LOCATION_REMOVED) as i64 * 200
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if eg:IsContains(e:GetHandler()) then return false end
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    if group::is_contains(&eg, h) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if aux.zptgroupcon(eg,nil,tc) then return true end
        if aux::zptgroupcon(d, &eg, None, tc, tp) {
            return true;
        }
    }
    // return false
    false
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.spreg(e,tp,eg,ep,ev,re,r,rp)`
fn spreg(d: &mut Duel, a: &EffArgs) {
    // if re and re:GetHandler()==e:GetHandler() then
    let h = effect::get_handler(d, a.e).nil();
    if let Some(re) = a.re {
        if effect::get_handler(d, re) == Some(h) {
            // e:SetLabel(Duel.GetTurnCount())
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, a.e, &[turn]);
            // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2)
            card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return e:GetLabelObject():GetLabel()~=Duel.GetTurnCount() and c:GetFlagEffect(id)>0
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) != duel::get_turn_count(d, None) as i64 && card::get_flag_effect(d, c, ID) > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // c:ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
