//! Advance Force
use super::prelude::*;

pub const ID: u32 = 38589847;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon with 1 tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetCondition(s.otcon)
    effect::set_condition(d, e2, Some(lua_fn(otcon)));
    // e2:SetTarget(aux.FieldSummonProcTg(s.ottg,s.ottgsum))
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(lua_fn(ottg)), Some(lua_fn(ottgsum)))));
    // e2:SetOperation(s.otop)
    effect::set_operation(d, e2, Some(lua_fn(otop)));
    // e2:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_SET_PROC)
    effect::set_code(d, e3, EFFECT_SET_PROC);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.otfilter(c,tp)`
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and (c:IsControler(tp) or c:IsFaceup())
    card::is_level_above(d, c, 5) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.otcon(e,c,minc)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if minc>1 then return false end
    let minc = a.int(2);
    if minc > 1 {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local mg=Duel.GetMatchingGroup(s.otfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return Duel.CheckTribute(c,1,1,mg)
    duel::check_tribute(d, c, 1, Some(1), Some(&mg), None, None)
}
/// `s.ottg(e,c)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    // return c:IsLevelAbove(7)
    let c = a.c(1);
    card::is_level_above(d, c, 7)
}
/// `s.ottgsum(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn ottgsum(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.c(9);
    // local mg=Duel.GetMatchingGroup(s.otfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let mut mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        // mg:Match(Card.IsControler,nil,tp)
        group::match_(d, &mut mg, &|d, x| card::is_controler(d, x, tp), NO_EX);
    }
    // local sg=Duel.SelectTribute(tp,c,1,1,mg,nil,nil,true)
    let sg = duel::select_tribute(d, tp, c, 1, 1, Some(&mg), None, None, true);
    // if sg then
    if let Some(sg) = sg {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not sg then return end
    let Some(sg) = sg else {
        return;
    };
    let c = a.c(8);
    // c:SetMaterial(sg)
    card::set_material(d, c, sg.clone());
    // Duel.Release(sg,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, sg, REASON_SUMMON | REASON_MATERIAL, None);
    // sg:DeleteGroup()
}
