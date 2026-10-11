//! Gizmek Yata, the Gleaming Vanguard
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 13224603;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // normal summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.nscon)
    effect::set_condition(d, e2, Some(condition(nscon)));
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // recover
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_RELEASE)
    effect::set_code(d, e3, EVENT_RELEASE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.rccon)
    effect::set_condition(d, e3, Some(condition(rccon)));
    // e3:SetTarget(s.rctg)
    effect::set_target(d, e3, Some(cost_target(rctg)));
    // e3:SetOperation(s.rcop)
    effect::set_operation(d, e3, Some(operation(rcop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // check
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetLabel(id)
        effect::set_label(d, ge1, &[ID as i64]);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // ge1:SetOperation(aux.sumreg)
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // ge2:SetLabel(id)
        effect::set_label(d, ge2, &[ID as i64]);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsSummonType,1,false,1,true,c,c:GetControler(),nil,false,e:GetHandler(),SUMMON_TYPE_NORMAL)
    let h = effect::get_handler(d, e).nil();
    duel::check_release_group(
        d,
        tp,
        Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL])),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, ..Default::default() },
        h.into(),
        false,
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(8);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsSummonType,1,1,false,true,true,c,nil,nil,false,e:GetHandler(),SUMMON_TYPE_NORMAL)
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL])),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: c, toplayer: None, zone: None, use_oppo: false, ..Default::default() },
        h.into(),
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
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g.clone(), REASON_COST, None);
        // g:DeleteGroup()
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonable(true,nil)
    card::is_summonable(d, c, true, None, 0, None)
}
// function s.nscon(e,tp,eg,ep,ev,re,r,rp)
fn nscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
// function s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
// function s.nsop(e,tp,eg,ep,ev,re,r,rp)
fn nsop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Summon(tp,tc,true,nil)
        duel::summon(d, a.tp, tc, true, None, 0, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.splimit(tc))
        effect::set_target(d, e1, Some(splimit(tc)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function s.splimit(tc)
fn splimit(tc: CardId) -> LuaFn {
    // return function(e,c,sump,sumtype,sumpos,targetp,se) return c:GetOriginalRace()~=tc:GetOriginalRace() end
    effect_card_fn(move |d, _e, c| card::get_original_race(d, c) != card::get_original_race(d, tc))
}
// function s.rccon(e,tp,eg,ep,ev,re,r,rp)
fn rccon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:GetSummonType()==SUMMON_TYPE_SPECIAL+1
    card::is_previous_location(d, c, LOCATION_MZONE) && card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
// function s.rctg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2050)
    duel::set_target_param(d, 2050);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,2050)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 2050);
    true
}
// function s.rcop(e,tp,eg,ep,ev,re,r,rp)
fn rcop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
