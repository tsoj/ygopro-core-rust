//! The Winged Dragon of Ra - Sphere Mode
use super::prelude::*;

pub const ID: u32 = 10000080;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_RA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), None, None);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_LIMIT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_LIMIT_SUMMON_PROC);
    // e2:SetTargetRange(POS_FACEUP_ATTACK,1)
    effect::set_target_range(d, e2, POS_FACEUP_ATTACK, 1);
    // e2:SetCondition(s.ttcon2)
    effect::set_condition(d, e2, Some(lua_fn(ttcon2)));
    // e2:SetTarget(s.tttg2)
    effect::set_target(d, e2, Some(lua_fn(tttg2)));
    // e2:SetOperation(s.ttop2)
    effect::set_operation(d, e2, Some(lua_fn(ttop2)));
    // e2:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=aux.AddNormalSetProcedure(c)
    let _e3 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // cannot special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e4, EFFECT_SPSUMMON_CONDITION);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // control return
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetOperation(s.retreg)
    effect::set_operation(d, e5, Some(operation(retreg)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // attack limit
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e6, EFFECT_CANNOT_ATTACK);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // cannot be target
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e7:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e7, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetValue(aux.imval2)
    effect::set_value_fn(d, e7, aux::imval2());
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=e7:Clone()
    let e8 = effect::clone(d, e7);
    // e8:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e8, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e8:SetValue(aux.tgoval)
    effect::set_value_fn(d, e8, aux::tgoval());
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // special summon
    // local e9=Effect.CreateEffect(c)
    let e9 = effect::create_effect(d, c);
    // e9:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e9, aux::stringid(ID, 2));
    // e9:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e9, CATEGORY_SPECIAL_SUMMON);
    // e9:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e9, EFFECT_TYPE_IGNITION);
    // e9:SetRange(LOCATION_MZONE)
    effect::set_range(d, e9, LOCATION_MZONE);
    // e9:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e9, Some(cost::self_tribute()));
    // e9:SetTarget(s.sptg)
    effect::set_target(d, e9, Some(cost_target(sptg)));
    // e9:SetOperation(s.spop)
    effect::set_operation(d, e9, Some(operation(spop)));
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
}
/// `s.retreg(e,tp,eg,ep,ev,re,r,rp)`
fn retreg(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,2)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 2, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetLabel(Duel.GetTurnCount()+1)
    effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64 + 1]);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetLabel() and e:GetOwner():GetFlagEffect(id)~=0
    duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e) && {
        let owner = effect::get_owner(d, a.e).nil();
        card::get_flag_effect(d, owner, ID) != 0
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetOwner()
    let c = effect::get_owner(d, a.e).nil();
    // c:ResetEffect(EFFECT_SET_CONTROL,RESET_CODE)
    card::reset_effect(d, c, EFFECT_SET_CONTROL, RESET_CODE);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e1, EFFECT_SET_CONTROL);
    // e1:SetValue(c:GetOwner())
    effect::set_value(d, e1, card::get_owner(d, c) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_TEMP_REMOVE|RESET_TURN_SET))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE | RESET_TURN_SET)), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_RA) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[CARD_RA]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetValue(4000)
            effect::set_value(d, e1, 4000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.ttcon2(e,c,minc,zone,relzone,exeff)`: the zone of `exeff`'s value, if it is a function
/// (`ret={ret(exeff,c)}`, `zone=(ret[2]>>16)&(ZONES_EMZ|ZONES_MMZ)`)
fn exeff_zone(d: &mut Duel, exeff: Option<EffectId>, c: CardId, zone: u32) -> u32 {
    let Some(exeff) = exeff else {
        return zone;
    };
    if let Value::Fn(f) = effect::get_value(d, exeff) {
        let ret = f.call(d, &[Value::Effect(exeff), Value::Card(c)]);
        if let Value::Multi(v) = ret {
            if v.len() > 1 {
                return ((v[1].int() >> 16) as u32) & (ZONES_EMZ | ZONES_MMZ);
            }
        }
    }
    zone
}
/// `function(e,c,minc,zone,relzone,exeff)` of `e2:SetCondition`: `s.ttcon2(...)`
fn ttcon2(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if minc>3 then return false end
    let minc = a.int(2);
    if minc > 3 {
        return false;
    }
    let mut zone = a.int(3) as u32;
    let relzone = a.int(4) as u32;
    let exeff = a.v(5).as_effect();
    // if exeff then ... end
    zone = exeff_zone(d, exeff, c, zone);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local mg=Duel.GetMatchingGroup(aux.IsZone,tp,0,LOCATION_MZONE,nil,relzone,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| aux::is_zone(d, x, relzone, tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // return Duel.CheckTribute(c,3,3,mg,1-tp,zone)
    duel::check_tribute(d, c, 3, Some(3), Some(&mg), Some(1 - tp), Some(zone))
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)` of `e2:SetTarget`: `s.tttg2(...)`
fn tttg2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    let mut zone = a.int(11) as u32;
    let relzone = a.int(12) as u32;
    let exeff = a.v(13).as_effect();
    // if exeff then ... end
    zone = exeff_zone(d, exeff, c, zone);
    // local mg=Duel.GetMatchingGroup(aux.IsZone,tp,0,LOCATION_MZONE,nil,relzone,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| aux::is_zone(d, x, relzone, tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // local g=Duel.SelectTribute(tp,c,3,3,mg,1-tp,zone,true)
    let g = duel::select_tribute(d, tp, c, 3, 3, Some(&mg), Some(1 - tp), Some(zone), true);
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // g:KeepAlive()
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)` of `e2:SetOperation`: `s.ttop2(...)`
fn ttop2(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, g, REASON_SUMMON | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
