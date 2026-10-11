//! Cosmic Tree Irmistil
use super::prelude::*;

pub const ID: u32 = 74139959;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 3, None, None, None, None);
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Gain LP equal to the ATK of a monster(s) Special Summoned by your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e,tp,eg) return Duel.IsMainPhase() and eg:IsExists(aux.FaceupFilter(Card.IsSummonPlayer,1-tp),1,nil) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetOperation(s.lpgainop)
    effect::set_operation(d, e1, Some(operation(lpgainop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card gain ATK based on how much LP you pay
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg) return Duel.IsMainPhase() and eg:IsExists(aux.FaceupFilter(Card.IsSummonPlayer,1-tp),1,nil) end` (condition of e1)
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // eg:IsExists(aux.FaceupFilter(Card.IsSummonPlayer,1-tp),1,nil)
    let tp = a.tp;
    duel::is_main_phase(d, None)
        && group::is_exists(d, &a.eg.clone().nil(), &aux::faceup_filter(move |d, c| card::is_summon_player(d, c, 1 - tp)), 1, NO_EX)
}
/// `function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end` (condition of e2)
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
/// `s.lpgainop(e,tp,eg,ep,ev,re,r,rp)`
fn lpgainop(d: &mut Duel, a: &EffArgs) {
    // local val=eg:Filter(aux.FaceupFilter(Card.IsSummonPlayer,1-tp),nil):GetSum(Card.GetAttack)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let f = group::filter(d, &eg, &aux::faceup_filter(move |d, c| card::is_summon_player(d, c, 1 - tp)), NO_EX);
    let val = group::get_sum(d, &f, &|d, c| card::get_attack(d, c) as i64);
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // if val>0 then
        if val > 0 {
            // Duel.Hint(HINT_CARD,1-tp,id)
            duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
            // Duel.Recover(tp,val,REASON_EFFECT)
            duel::recover(d, a.tp, val, REASON_EFFECT, false, None);
        }
    } else {
        // local eff=e:GetLabelObject()
        let eff = effect::get_label_object(d, a.e).as_effect();
        // if eff and not eff:IsDeleted() then
        let live = match eff {
            Some(eff) if !effect::is_deleted(d, eff) => Some(eff),
            _ => None,
        };
        if let Some(eff) = live {
            // eff:SetLabel(eff:GetLabel()+val)
            let label = effect::get_label(d, eff);
            effect::set_label(d, eff, &[label + val]);
        } else {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetOperation(s.chainsolvedop)
            effect::set_operation(d, e1, Some(operation(chainsolvedop)));
            // e1:SetLabel(val)
            effect::set_label(d, e1, &[val]);
            // e1:SetLabelObject(e)
            effect::set_label_object(d, e1, Value::Effect(a.e));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // e:SetLabelObject(e1)
            effect::set_label_object(d, a.e, Value::Effect(e1));
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
            // e2:SetOperation(function() e:SetLabelObject(nil) end)
            let ea = a.e;
            effect::set_operation(d, e2, Some(LuaFn::new(move |d, _a| {
                effect::set_label_object(d, ea, Value::Nil);
                Value::Nil
            })));
            // e2:SetReset(RESET_CHAIN)
            effect::set_reset(d, e2, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
        }
    }
}
/// `s.chainsolvedop(e,tp,eg,ep,ev,re,r,rp)`
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    // local val=e:GetLabel()
    let val = effect::get_label(d, a.e);
    // if val>0 then
    if val > 0 {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.Recover(tp,val,REASON_EFFECT)
        duel::recover(d, a.tp, val, REASON_EFFECT, false, None);
    }
    // e:Reset()
    effect::reset(d, a.e);
    // e:GetLabelObject():SetLabelObject(nil)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, lo, Value::Nil);
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,1000) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 1000);
    }
    // local cost_options={}
    // for i=1,math.floor(math.min(Duel.GetLP(tp),3000)/1000) do cost_options[i]=i*1000 end
    let lp = duel::get_lp(d, a.tp) as i64;
    let n = lp.min(3000).div_euclid(1000);
    let cost_options: Vec<u64> = (1..=n).map(|i| (i * 1000) as u64).collect();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local lp_cost=Duel.AnnounceNumber(tp,cost_options)
    let lp_cost = duel::announce_number(d, a.tp, &cost_options).nil().0 as i64;
    // e:SetLabel(lp_cost)
    effect::set_label(d, a.e, &[lp_cost]);
    // Duel.PayLPCost(tp,lp_cost)
    duel::pay_lp_cost(d, a.tp, lp_cost as u32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
