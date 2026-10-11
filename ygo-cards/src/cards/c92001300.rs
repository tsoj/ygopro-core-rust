//! Ancient Gear Castle
//! 古代の機械城
use super::prelude::*;

pub const ID: u32 = 92001300;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0xb)
    card::enable_counter_permit(d, c, 0xb, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ANCIENT_GEAR))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]))));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // counter
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetOperation(s.addc)
    effect::set_operation(d, e3, Some(operation(addc)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_MSET)
    effect::set_code(d, e4, EVENT_MSET);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // summon proc
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function()
    //     --summon proc
    //     ...
    // end)
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, ge1, aux::stringid(ID, 0));
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetCode(EFFECT_SUMMON_PROC)
        effect::set_code(d, ge1, EFFECT_SUMMON_PROC);
        // ge1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
        effect::set_target_range(d, ge1, LOCATION_HAND, LOCATION_HAND);
        // ge1:SetCondition(s.sumcon)
        effect::set_condition(d, ge1, Some(lua_fn(sumcon)));
        // ge1:SetTarget(aux.FieldSummonProcTg(aux.TargetBoolFunction(Card.IsSetCard,SET_ANCIENT_GEAR),s.sumtg))
        effect::set_target(d, ge1, Some(aux::field_summon_proc_tg(Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]))), Some(lua_fn(sumtg)))));
        // ge1:SetOperation(s.sumop)
        effect::set_operation(d, ge1, Some(lua_fn(sumop)));
        // ge1:SetValue(SUMMON_TYPE_TRIBUTE)
        effect::set_value(d, ge1, SUMMON_TYPE_TRIBUTE as i64);
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0xb,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0xb, 1, false);
}
/// `s.castlefilter(c,tp,mi,ma)`
fn castlefilter(d: &mut Duel, c: CardId, tp: u8, mi: i32) -> bool {
    // return c:IsHasEffect(id) and c:GetCounter(0xb)>=mi and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsReleasable()
    !card::is_has_effect(d, c, ID, None).is_empty()
        && card::get_counter(d, c, 0xb) >= mi
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_releasable(d, c, None, None)
}
/// `s.sumcon(e,c,minc)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let minc = a.int(2) as i32;
    // local mi,ma=c:GetTributeRequirement()
    let (mut mi, ma) = card::get_tribute_requirement(d, c);
    // if mi<minc then mi=minc end
    if mi < minc {
        mi = minc;
    }
    // if ma<mi then return false end
    if ma < mi {
        return false;
    }
    // return ma>0 and Duel.IsExistingMatchingCard(s.castlefilter,c:GetControler(),LOCATION_SZONE,0,1,nil,c:GetControler(),mi,ma)
    let tp = card::get_controler(d, c);
    ma > 0 && duel::is_existing_matching_card(d, Some(&|d, x| castlefilter(d, x, tp, mi)), tp, LOCATION_SZONE, 0, 1, NO_EX)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    // tp=c:GetControler()
    let c = a.c(9);
    let tp = card::get_controler(d, c);
    // local mi,ma=c:GetTributeRequirement()
    let (mi, _ma) = card::get_tribute_requirement(d, c);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.castlefilter,tp,LOCATION_SZONE,0,1,1,true,nil,tp,mi,ma)
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| castlefilter(d, x, tp, mi)), tp, LOCATION_SZONE, 0, 1, 1, true, NO_EX);
    // if sg then
    if let Some(sg) = sg {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, a.e(0), Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sumop(d: &mut Duel, a: Args) {
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not sg then return end
    let Some(sg) = sg else {
        return;
    };
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    // sg:DeleteGroup()
}
