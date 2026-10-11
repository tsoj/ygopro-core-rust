//! Kewl Tune Loudness War
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 41069676;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KEWL_TUNE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ Tuners
    // Synchro.AddProcedure(c,nil,1,1,s.tunerfilter,1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(mat_filter(tunerfilter)), 1, 99, None, None, None, None, None);
    // Other Tuners you control cannot be destroyed by card effects, also your opponent cannot target them with card effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1a, LOCATION_MZONE, 0);
    // e1a:SetTarget(function(e,c) return c:IsType(TYPE_TUNER) and c~=e:GetHandler() end)
    effect::set_target(d, e1a, Some(effect_card_fn(|d, e, c| card::is_type(d, c, TYPE_TUNER) && c != effect::get_handler(d, e).nil())));
    // e1a:SetValue(1)
    effect::set_value(d, e1a, 1);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1b:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1b, aux::tgoval());
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Apply the effect of 1 "Kewl Tune" monster in your GY that activates if sent to the GY as Synchro Material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.applycost)
    effect::set_cost(d, e2, Some(cost_target(applycost)));
    // e2:SetTarget(s.applytg)
    effect::set_target(d, e2, Some(cost_target(applytg)));
    // e2:SetOperation(s.applyop)
    effect::set_operation(d, e2, Some(operation(applyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Multiple Tuners
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_MULTIPLE_TUNERS)
    effect::set_code(d, e3, EFFECT_MULTIPLE_TUNERS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tunerfilter(c,scard,sumtype,tp)
fn tunerfilter(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER,scard,sumtype,tp) or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    card::is_type_ctx(d, c, TYPE_TUNER, scard, sumtype, Some(tp)) || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
// Lua call of an EVENT_BE_MATERIAL effect's target:
// tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0)
fn call_be_material_target(d: &mut Duel, eff: EffectId, tp: u8, e: EffectId) -> bool {
    match effect::get_target(d, eff) {
        None => true,
        Some(tg) => tg
            .call(
                d,
                &[
                    Value::Effect(eff),
                    Value::Int(tp as i64),
                    Value::Group(CardSet::new()),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                    Value::Effect(e),
                    Value::Int(REASON_EFFECT as i64),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                ],
            )
            .truthy(),
    }
}
// function s.rmfilter(c,e,tp)
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_KEWL_TUNE) and c:IsMonster() and c:IsAbleToRemoveAsCost()) then return false end
    if !(card::is_set_card(d, c, &[SET_KEWL_TUNE as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)) {
        return false;
    }
    // local effs={c:GetOwnEffects()}
    let effs = card::get_own_effects(d, c);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if eff:GetCode()==EVENT_BE_MATERIAL then
        if effect::get_code(d, eff) == EVENT_BE_MATERIAL {
            // local tg=eff:GetTarget()
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            if call_be_material_target(d, eff, tp, e) {
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
// function s.applycost(e,tp,eg,ep,ev,re,r,rp,chk)
fn applycost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let rcg = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &rcg).nil();
    // Duel.Remove(rc,POS_FACEUP,REASON_COST)
    duel::remove(d, rc, POS_FACEUP as u8, REASON_COST, None, None);
    // local cd=e:GetChainData()
    // cd.available_effects={}
    let mut available: Vec<Value> = Vec::new();
    // local effs={rc:GetOwnEffects()}
    let effs = card::get_own_effects(d, rc);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if eff:GetCode()==EVENT_BE_MATERIAL then
        if effect::get_code(d, eff) == EVENT_BE_MATERIAL {
            // local tg=eff:GetTarget()
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            if call_be_material_target(d, eff, tp, a.e) {
                // table.insert(cd.available_effects,eff)
                available.push(Value::Effect(eff));
            }
        }
    }
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Multi(available));
    true
}
// function s.applytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local eff=e:GetLabelObject()
        // return eff and eff:GetTarget() and eff:GetTarget()(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        let eff = effect::get_label_object(d, a.e).as_effect();
        return match eff {
            None => false,
            Some(eff) => match effect::get_target(d, eff) {
                None => false,
                Some(tg) => tg.call(d, &target_args(a, Some(0), Some(chkc))).truthy(),
            },
        };
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local eff=nil
    // local cd=e:GetChainData()
    let available = chain::effect_data(d, a.e, None, chain::field::CARD);
    let list = match &available {
        Value::Multi(v) => v.clone(),
        _ => Vec::new(),
    };
    let effs: Vec<EffectId> = list.iter().filter_map(|v| v.as_effect()).collect();
    // if #cd.available_effects>1 then
    let eff: Option<EffectId> = if effs.len() > 1 {
        // local available_effs_desc={}
        // for _,eff in ipairs(cd.available_effects) do table.insert(available_effs_desc,eff:GetDescription()) end
        let descs: Vec<u64> = effs.iter().map(|&x| effect::get_description(d, x)).collect();
        // local op=Duel.SelectOption(tp,table.unpack(available_effs_desc))
        let op = duel::select_option(d, tp, true, &descs);
        // eff=cd.available_effects[op+1]
        Some(effs[op as usize])
    } else {
        // eff=cd.available_effects[1]
        effs.first().copied()
    };
    let eff = eff.nil();
    // Duel.Hint(HINT_OPSELECTED,1-tp,eff:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, eff));
    // e:SetLabel(eff:GetLabel())
    let lbl = effect::get_label(d, eff);
    effect::set_label(d, a.e, &[lbl]);
    // e:SetLabelObject(eff:GetLabelObject())
    let lo = effect::get_label_object(d, eff);
    effect::set_label_object(d, a.e, lo);
    // e:SetProperty(eff:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and EFFECT_FLAG_CARD_TARGET or 0)
    let prop = if effect::is_has_property(d, eff, EFFECT_FLAG_CARD_TARGET, 0) { EFFECT_FLAG_CARD_TARGET } else { 0 };
    effect::set_property(d, a.e, prop, 0);
    // local tg=eff:GetTarget()
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = effect::get_target(d, eff) {
        tg.call(d, &target_args(a, Some(1), None));
    }
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    // cd.loudness_war_target_effect=eff
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Effect(eff));
    true
}
// The arguments (e,tp,eg,ep,ev,re,r,rp[,chk[,chkc]]) of a Lua target call
fn target_args(a: &EffArgs, chk: Option<i64>, chkc: Option<CardId>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    if let Some(chkc) = chkc {
        v.push(Value::Card(chkc));
    }
    v
}
// function s.applyop(e,tp,eg,ep,ev,re,r,rp)
fn applyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local eff=e:GetChainData().loudness_war_target_effect
    let eff = chain::effect_data(d, a.e, None, chain::field::CARD + 1).as_effect();
    // if not eff then return end
    let Some(eff) = eff else {
        return;
    };
    // local op=eff:GetOperation()
    // if op then op(e,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE) end
    if let Some(op) = effect::get_operation(d, eff) {
        op.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(tp as i64),
                Value::Group(CardSet::new()),
                Value::Int(PLAYER_NONE as i64),
                Value::Int(0),
                Value::Effect(a.e),
                Value::Int(REASON_EFFECT as i64),
                Value::Int(PLAYER_NONE as i64),
            ],
        );
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end
fn e2_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
