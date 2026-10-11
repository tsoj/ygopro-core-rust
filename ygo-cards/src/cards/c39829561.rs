//! Destiny HERO - Departed
use super::prelude::*;

pub const ID: u32 = 39829561;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish when destroyed by battle or sent to the GY from the Deck or Hand by an effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetValue(s.redirectval)
    effect::set_value_fn(d, e2, lua_fn(redirectval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Infinite loop prevention when only forced triggers are into play (Slifer the Sky Dragon, King Tiger Wanghu, etc.)
    // local departchktable={}
    // departchktable[-1]={}
    // departchktable[0]=0
    // departchktable[-1][0]=0
    // local ge1=Effect.CreateEffect(c)
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // ge1:SetCode(EVENT_CHAINING)
    effect::set_code(d, ge1, EVENT_CHAINING);
    // ge1:SetLabelObject({e1,departchktable})
    // The table lives in the label object of e1 (see `get_table`); ge1 refers to e1.
    effect::set_label_object(d, ge1, Value::Effect(e1));
    // ge1:SetOperation(s.checkop)
    effect::set_operation(d, ge1, Some(operation(checkop)));
    // Duel.RegisterEffect(ge1,0)
    duel::register_effect(d, ge1, 0);
    // e1:SetLabelObject(departchktable)
    set_table(d, e1, &[], &[]);
}

/// The Lua table `departchktable`: `departchktable[1..departchktable[0]]` is `cur`, and
/// `departchktable[-1][1..departchktable[-1][0]]` is `prev`. Stored as the label object of `e1`.
fn get_table(d: &Duel, e1: EffectId) -> (Vec<EffectId>, Vec<EffectId>) {
    let ids = |v: &Value| -> Vec<EffectId> {
        match v {
            Value::Multi(xs) => xs.iter().filter_map(|x| x.as_effect()).collect(),
            _ => Vec::new(),
        }
    };
    match effect::get_label_object(d, e1) {
        Value::Multi(parts) if parts.len() == 2 => (ids(&parts[0]), ids(&parts[1])),
        _ => (Vec::new(), Vec::new()),
    }
}
fn set_table(d: &mut Duel, e1: EffectId, cur: &[EffectId], prev: &[EffectId]) {
    let enc = |xs: &[EffectId]| Value::Multi(xs.iter().map(|e| Value::Effect(*e)).collect());
    effect::set_label_object(d, e1, Value::Multi(vec![enc(cur), enc(prev)]));
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsTurnPlayer(tp) then return false end
    if !duel::is_turn_player(d, a.tp) {
        return false;
    }
    // local departchktable=e:GetLabelObject()
    let (cur, prev) = get_table(d, a.e);
    // if departchktable[0]~=departchktable[-1][0] or departchktable[1]==nil then return true end
    if cur.len() != prev.len() || cur.is_empty() {
        return true;
    }
    // for i=1,departchktable[0] do
    for i in 0..cur.len() {
        // if departchktable[i]~=departchktable[-1][i] then return true end
        if cur[i] != prev[i] {
            return true;
        }
    }
    // return false
    false
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,1-tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, h, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
// function s.redirectval(e,c,reason)
fn redirectval(d: &mut Duel, a: Args) -> i64 {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    let reason = a.int(2) as u32;
    // if (c:IsLocation(LOCATION_MZONE) and (reason&REASON_BATTLE)==REASON_BATTLE) or
    // (c:IsLocation(LOCATION_DECK|LOCATION_HAND) and (reason&REASON_EFFECT)==REASON_EFFECT) then
    if (card::is_location(d, c, LOCATION_MZONE) && (reason & REASON_BATTLE) == REASON_BATTLE)
        || (card::is_location(d, c, LOCATION_DECK | LOCATION_HAND) && (reason & REASON_EFFECT) == REASON_EFFECT)
    {
        // return LOCATION_REMOVED
        return LOCATION_REMOVED as i64;
    }
    // return 0
    0
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local labelEff,departchktable=table.unpack(e:GetLabelObject())
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let (mut cur, mut prev) = get_table(d, e1);
    // if re==labelEff then
    if a.re == Some(e1) {
        // if departchktable[1]==nil then
        if cur.is_empty() {
            // departchktable[1]=re
            // departchktable[0]=1
            cur = vec![e1];
        } else {
            // for i=1,departchktable[0] do departchktable[-1][i]=departchktable[i] end
            prev = cur.clone();
            // departchktable[1]=re
            // departchktable[-1][0]=departchktable[0]
            // departchktable[0]=1
            cur = vec![e1];
        }
    } else {
        // local re = the chained effect (`re:IsHasType` indexes it: nil is an error)
        let re = a.re.nil();
        // if not re:IsHasType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_QUICK_F) then
        if !effect::is_has_type(d, re, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_QUICK_F) {
            // for i=1,departchktable[0] do departchktable[-1][i]=nil departchktable[i]=nil end
            // departchktable[-1][0]=0
            // departchktable[0]=0
            cur.clear();
            prev.clear();
        } else if !cur.is_empty() {
            // elseif departchktable[0]>0 then
            // departchktable[0]=departchktable[0]+1
            // departchktable[departchktable[0]]=re
            cur.push(re);
        }
    }
    set_table(d, e1, &cur, &prev);
}
