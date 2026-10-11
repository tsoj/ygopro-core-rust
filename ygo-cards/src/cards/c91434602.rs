//! Majesty Hyperion
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91434602;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_AGENT], listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Battle damage also inflicted to opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ALSO_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_ALSO_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_FAIRY))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FAIRY))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish card from either GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetCost(s.rmcost)
    effect::set_cost(d, e3, Some(cost_target(rmcost)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spfilter(c)
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THE_AGENT) and c:IsAbleToRemoveAsCost()
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE)) and aux.SpElimFilter(c,true,true)
    card::is_set_card(d, c, &[SET_THE_AGENT as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
        && aux::sp_elim_filter(d, c, true, true)
}
// function s.hspcon(e,c)
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    let eff = card::get_card_effect(d, c, EFFECT_NECRO_VALLEY);
    // for _,te in ipairs(eff) do
    for te in eff {
        // local op=te:GetOperation()
        // if not op or op(e,c) then return false end
        match effect::get_operation(d, te) {
            None => return false,
            Some(op) => {
                if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    let rs = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&rs), 0, tp, None, None, None, false), Err(true))
}
// function s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g<1 then return false end
    if g.is_empty() {
        return false;
    }
    // g:KeepAlive()
    // e:SetLabelObject(g)
    effect::set_label_object(d, e, Value::Group(g));
    // return true
    true
}
// function s.hspop(e,tp,eg,ep,ev,re,r,rp,c)
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),e:GetHandlerPlayer(),LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
    //     or Duel.IsEnvironment(CARD_SANCTUARY_SKY) then
    let sky = duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_SANCTUARY_SKY]))),
        hp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        NO_EX,
    ) || duel::is_environment(d, CARD_SANCTUARY_SKY, None, None);
    let h = effect::get_handler(d, a.e).nil();
    if sky {
        // return e:GetHandler():GetFlagEffect(id)<2
        card::get_flag_effect(d, h, ID) < 2
    } else {
        // else return e:GetHandler():GetFlagEffect(id)<1 end
        card::get_flag_effect(d, h, ID) < 1
    }
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    //     and Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,c)
    if !(card::is_race(d, c, RACE_FAIRY)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false)))
    {
        return false;
    }
    duel::is_existing_target(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, c.into())
}
// function s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE|LOCATION_MZONE|LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE|LOCATION_MZONE|LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && rmfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc.filter(|&tc| card::is_relate_to_effect(d, tc, a.e)) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
