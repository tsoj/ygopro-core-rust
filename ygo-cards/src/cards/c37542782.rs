//! Cyberdark End Dragon
use super::prelude::*;

pub const ID: u32 = 37542782;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40418351, 1546123], listed_series: &[SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,40418351,1546123)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(40418351), fusion::Mat::Code(1546123)]);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(lua_fn(hspcon)));
    // e2:SetTarget(s.hsptg)
    effect::set_target(d, e2, Some(lua_fn(hsptg)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(lua_fn(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // immune
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.immval)
    effect::set_value_fn(d, e3, lua_fn(immval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // equip
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e4, CATEGORY_EQUIP);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.eqtg)
    effect::set_target(d, e4, Some(cost_target(eqtg)));
    // e4:SetOperation(s.eqop)
    effect::set_operation(d, e4, Some(operation(eqop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e4)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equip_limit_op), e4, None, None, None);
    // multi
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e5, EFFECT_EXTRA_ATTACK);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetValue(s.atkval)
    effect::set_value_fn(d, e5, lua_fn(atkval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.hspfilter(c,tp,sc)`
fn hspfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return c:IsType(TYPE_FUSION,sc,MATERIAL_FUSION,tp) and c:IsSetCard(SET_CYBERDARK,sc,MATERIAL_FUSION,tp) and c:IsLevelBelow(10)
    //     and c:GetEquipGroup():IsExists(aux.FilterBoolFunction(Card.IsCode,1546123),1,nil)
    //     and c:IsControler(tp) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    if !card::is_type_ctx(d, c, TYPE_FUSION, Some(sc), MATERIAL_FUSION as u64, Some(tp)) {
        return false;
    }
    if !card::is_set_card_ctx(d, c, &[SET_CYBERDARK as u16], Some(sc), MATERIAL_FUSION as u64, Some(tp)) {
        return false;
    }
    if !card::is_level_below(d, c, 10) {
        return false;
    }
    let eq = card::get_equip_group(d, c);
    if !group::is_exists(d, &eq, &|d, x| card::is_code(d, x, &[1546123]), 1, NO_EX) {
        return false;
    }
    if !card::is_controler(d, c, tp) {
        return false;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `function(e,c)` of e2:SetCondition
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.hspfilter,1,false,1,true,c,tp,nil,false,nil,tp,c)
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| hspfilter(d, x, tp, c)),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c)` of e2:SetTarget
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.hspfilter,1,1,false,true,true,c,nil,nil,false,nil,tp,c)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| hspfilter(d, x, tp, c)),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: Some(c), ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of e2:SetOperation
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated()
    let owner = effect::get_owner_player(d, te);
    if owner == effect::get_handler_player(d, e) {
        return false;
    }
    effect::is_activated(d, te)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsForbidden()
    card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true));
}
/// `function(ec,c,tp) return ec:IsControler(tp) end` (equipval of aux.AddEREquipLimit)
fn eqval(d: &mut Duel, a: Args) -> bool {
    card::is_controler(d, a.c(0), a.p(2))
}
/// `s.equipop` as called by aux.AddEREquipLimit: `function(c,e,tp,tc)`
fn equip_limit_op(d: &mut Duel, a: Args) {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let nv = aux::necro_valley_filter(|d, x| filter(d, x));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_equip_count(d, c) > 0
}
/// `s.atkval(e)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // return e:GetHandler():GetEquipCount()-1
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_equip_count(d, c) as i64 - 1
}
