//! Duralume, Vaalmonican Heathen Hallow
use super::prelude::*;

pub const ID: u32 = 76821171;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VAALMONICA], counter_list: &[COUNTER_RESONANCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Effect Monster
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),1,1,nil,nil,s.splimit)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 1, Some(1), None, None, Some(lua_fn(splimit)));
    // Cannot be Link Summoned unless you have a Fairy Monster Card with 3 or more Resonance Counters in your Pendulum Zone
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can only Special Summon "Duralume, Vaalmonican Heathen Hallow" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Destroy opponent's monsters up to the number of Resonance Counters in your Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card can make a 2nd and 3rd attack this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.tripleatkcost)
    effect::set_cost(d, e2, Some(cost_target(tripleatkcost)));
    // e2:SetTarget(s.tripleatktg)
    effect::set_target(d, e2, Some(cost_target(tripleatktg)));
    // e2:SetOperation(s.tripleatkop)
    effect::set_operation(d, e2, Some(operation(tripleatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let sp = a.p(2);
    let st = a.int(3);
    // return (st&SUMMON_TYPE_LINK)~=SUMMON_TYPE_LINK
    if (st & SUMMON_TYPE_LINK as i64) != SUMMON_TYPE_LINK as i64 {
        return true;
    }
    // or Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsOriginalRace,RACE_FAIRY),sp,LOCATION_PZONE,0,nil):GetSum(Card.GetCounter,COUNTER_RESONANCE)>=3
    let f = aux::faceup_filter(|d, x| card::is_original_race(d, x, RACE_FAIRY));
    let g = duel::get_matching_group(d, Some(&f), sp, LOCATION_PZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_counter(d, x, COUNTER_RESONANCE as u16) as i64) >= 3
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end` (e1)
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `function() return Duel.IsAbleToEnterBP() end` (e2)
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // local ct=Duel.GetCounter(tp,1,0,COUNTER_RESONANCE)
    let ct = duel::get_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tg=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,ct,nil)
    let tg = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then Duel.Destroy(tg,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.tripleatkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tripleatkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_COST);
    true
}
/// `s.tripleatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tripleatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local effs={e:GetHandler():IsHasEffect(EFFECT_EXTRA_ATTACK)}
        let c = effect::get_handler(d, a.e).nil();
        let effs = card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None);
        // for _,eff in ipairs(effs) do
        for eff in effs {
            // if eff:GetValue()>=2 then return false end
            if effect::get_value(d, eff).int() >= 2 {
                return false;
            }
        }
        // return true
        return true;
    }
    true
}
/// `s.tripleatkop(e,tp,eg,ep,ev,re,r,rp)`
fn tripleatkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(2)
        effect::set_value(d, e1, 2);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
