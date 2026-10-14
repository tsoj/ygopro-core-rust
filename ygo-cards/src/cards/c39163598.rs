//! Planet Pollutant Virus
//! 惑星汚染ウイルス
use super::prelude::*;

pub const ID: u32 = 39163598;
pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_DRAW_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //   s[0]={}
    //   aux.AddValuesReset(function() ... end)
    // end)
    aux::global_check(d, ID, |d| {
        set_list(d, Vec::new());
        aux::add_values_reset(d, reset_all);
    });
}
/// The list `s[0]` of the reset effects (`e4`), kept in the duel.
fn list_get(d: &Duel) -> Vec<Value> {
    match s_var(d, ID, 0) {
        Value::Multi(v) => v,
        _ => Vec::new(),
    }
}
fn set_list(d: &mut Duel, list: Vec<Value>) {
    set_s_var(d, ID, 0, Value::Multi(list));
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,nil,SET_ALIEN) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_ALIEN as u16])), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,nil,SET_ALIEN)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_ALIEN as u16])), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetCounter(COUNTER_A)==0
    card::is_faceup(d, c) && card::get_counter(d, c, COUNTER_A as u16) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.ctop1)
    effect::set_operation(d, e1, Some(operation(ctop1)));
    // (ygoai override lua_fixes/c39163598.lua: no e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3) here and for
    // e4: the core deleted e1-e4 in the opponent's 3rd End Phase, before s.reset's cleanup at the turn end,
    // which then raised on the deleted e4; s.reset alone ends them)
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetOperation(s.ctop2)
    effect::set_operation(d, e3, Some(operation(ctop2)));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // local descnum=tp==e:GetHandler():GetOwner() and 0 or 1
    let owner = card::get_owner(d, c);
    let descnum: u32 = if tp == owner { 0 } else { 1 };
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetDescription(aux.Stringid(id,descnum))
    effect::set_description(d, e4, aux::stringid(ID, descnum));
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(1082946)
    effect::set_code(d, e4, 1082946);
    // e4:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e4, tp);
    // e4:SetLabel(0)
    effect::set_label(d, e4, &[0]);
    // e4:SetOperation(s.reset)
    effect::set_operation(d, e4, Some(operation(reset_op)));
    // (ygoai override lua_fixes/c39163598.lua: no e4:SetReset, see e1)
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // table.insert(s[0],e4)
    let mut list = list_get(d);
    list.push(Value::Effect(e4));
    set_list(d, list);
    // s[0][e4]={e1,e2,e3}
    effect::set_label_object(d, e4, Value::Multi(vec![Value::Effect(e1), Value::Effect(e2), Value::Effect(e3)]));
}
/// `s.ctop1(e,tp,eg,ep,ev,re,r,rp)`
fn ctop1(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp then eg:GetFirst():AddCounter(COUNTER_A,1) end
    if a.ep != a.tp {
        let eg = a.eg.clone().nil();
        let tc = group::get_first(d, &eg).nil();
        card::add_counter(d, tc, COUNTER_A as u16, 1, false);
    }
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsFaceup() and not tc:IsSummonPlayer(tp) then tc:AddCounter(COUNTER_A,1) end
        if card::is_faceup(d, tc) && !card::is_summon_player(d, tc, tp) {
            card::add_counter(d, tc, COUNTER_A as u16, 1, false);
        }
    }
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)` as the operation of `e4`
fn reset_op(d: &mut Duel, a: &EffArgs) {
    reset_inner(d, a.e, a.ev);
}
/// The body of `s.reset`: count the turns of the reset effect `e` and reset it after three.
fn reset_inner(d: &mut Duel, e: EffectId, ev: u32) {
    // local label=e:GetLabel()
    // label=label+1
    let label = effect::get_label(d, e) + 1;
    // e:SetLabel(label)
    effect::set_label(d, e, &[label]);
    // if ev==1082946 then e:GetOwner():SetTurnCounter(label) end
    let owner_c = effect::get_owner(d, e).nil();
    if ev == 1082946 {
        card::set_turn_counter(d, owner_c, label as u16);
    }
    // e:GetOwner():SetTurnCounter(0)
    card::set_turn_counter(d, owner_c, 0);
    // if label==3 then
    if label == 3 {
        // local e1,e2,e3=table.unpack(s[0][e])
        let parts: Vec<EffectId> = match effect::get_label_object(d, e) {
            Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
            _ => Vec::new(),
        };
        // e:Reset()
        effect::reset(d, e);
        // if e1 then e1:Reset() end ... (e2, e3)
        for p in parts {
            effect::reset(d, p);
        }
        // s[0][e]=nil
        effect::set_label_object(d, e, Value::Nil);
        // for i,te in ipairs(s[0]) do if te==e then table.remove(s[0],i) break end end
        let mut list = list_get(d);
        if let Some(pos) = list.iter().position(|v| v.as_effect() == Some(e)) {
            list.remove(pos);
        }
        set_list(d, list);
    }
}
/// The reset function of `aux.AddValuesReset`
fn reset_all(d: &mut Duel) {
    // for _,te in ipairs(s[0]) do
    // (ipairs reads the list again at every step, as the Lua loop does)
    let mut i = 0usize;
    loop {
        let list = list_get(d);
        if i >= list.len() {
            break;
        }
        let te = list[i].as_effect().nil();
        // (ygoai override lua_fixes/c39163598.lua: e4 has no core reset any more, so te is never a
        // deleted effect here)
        // if Duel.GetTurnPlayer()~=te:GetOwnerPlayer() then
        let owner_p = effect::get_owner_player(d, te);
        if duel::get_turn_player(d) != owner_p {
            // s.reset(te,te:GetOwnerPlayer(),nil,0,0,nil,0,0)
            reset_inner(d, te, 0);
        }
        i += 1;
    }
}
