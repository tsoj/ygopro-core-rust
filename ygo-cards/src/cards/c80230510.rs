//! Tribute Burial
use super::prelude::*;

pub const ID: u32 = 80230510;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCondition(s.otcon)
    effect::set_condition(d, e1, Some(lua_fn(otcon)));
    // e1:SetTarget(aux.FieldSummonProcTg(s.ottg,s.sumtg))
    effect::set_target(d, e1, Some(aux::field_summon_proc_tg(Some(lua_fn(ottg)), Some(lua_fn(sumtg)))));
    // e1:SetOperation(s.otop)
    effect::set_operation(d, e1, Some(operation(otop)));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e1, SUMMON_TYPE_TRIBUTE as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_PROC)
    effect::set_code(d, e2, EFFECT_SET_PROC);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.otcon(e,c,minc)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return minc<=2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let minc = a.int(2);
    minc <= 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
}
/// `s.ottg(e,c)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    // local mi,ma=c:GetTributeRequirement()
    let c = a.c(1);
    let (mi, ma) = card::get_tribute_requirement(d, c);
    // return mi<=2 and ma>=2
    mi <= 2 && ma >= 2
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local mg1=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let mg1 = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local mg2=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let mg2 = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    // ::restart::
    loop {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g1=mg1:Select(tp,1,1,true,nil)
        // if not g1 then return false end
        let Some(mut g1) = group::select(d, &mg1, tp, 1, 1, true, NO_EX) else {
            return false;
        };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local tc=mg2:SelectUnselect(g1,tp,false,false,2,2)
        let tc = group::select_unselect(d, &mg2, Some(&g1), tp, false, false, Some(2), Some(2));
        // if mg2:IsContains(tc) then
        if tc.is_some_and(|tc| group::is_contains(&mg2, tc)) {
            let tc = tc.unwrap();
            // g1:AddCard(tc)
            group::add_card(&mut g1, &CardSet::from_card(tc));
            // g1:KeepAlive()
            // e:SetLabelObject(g1)
            effect::set_label_object(d, e, Value::Group(g1));
            // return true
            return true;
        }
        // goto restart
    }
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: &EffArgs) {
    // local sg=e:GetLabelObject()
    // if not sg then return end
    let Some(sg) = effect::get_label_object(d, a.e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    // sg:DeleteGroup()
    // Duel.ResetFlagEffect(tp,id)
    duel::reset_flag_effect(d, a.tp, ID);
}
