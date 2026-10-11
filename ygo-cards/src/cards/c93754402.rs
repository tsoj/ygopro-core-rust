//! Fire Formation - Domei
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 93754402;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When activated, ritual summon 1 beast-warrior ritual monster from hand
    // Ritual.Target(s.filter,RITPROC_GREATER) / Ritual.Operation(s.filter,RITPROC_GREATER)
    let rp = ritual::RitualProc { filter: Some(std::sync::Arc::new(filter)), lvtype: RITPROC_GREATER, ..Default::default() };
    let tg = ritual::target(d, rp.clone());
    let op = ritual::operation(d, rp);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target(Ritual.Target(...),Ritual.Operation(...)))
    effect::set_target(d, e1, Some(target(tg, op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this face-up card is sent to GY, special summon 1 "Fire Fist" monster from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsRitualMonster()
    card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_ritual_monster(d, c)
}
/// The arguments of the Lua `(e,tp,eg,ep,ev,re,r,rp)` of the effect, as the core passes them
fn base_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `tg(e,tp,eg,ep,ev,re,r,rp,chk)` for a chk of 0 or 1
fn call_chk(d: &mut Duel, f: &LuaFn, a: &EffArgs, chk: i64) -> Value {
    let mut args = base_args(a);
    args.push(Value::Int(chk));
    f.call(d, &args)
}
/// `s.target(tg,op)`: the target of the effect; it chooses whether to Ritual Summon
fn target(tg: LuaFn, op: LuaFn) -> LuaFn {
    cost_target(move |d, a| {
        // if chk==0 then return true end
        if a.chk == Some(0) {
            return true;
        }
        // if tg(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if call_chk(d, &tg, a, 0).truthy() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
            // e:SetOperation(s.rop(op))
            effect::set_operation(d, a.e, Some(rop(op.clone())));
            // tg(e,tp,eg,ep,ev,re,r,rp,1)
            call_chk(d, &tg, a, 1);
        }
        true
    })
}
/// `s.rop(op)`: the operation, which does nothing when the card is no longer related to the effect
fn rop(op: LuaFn) -> LuaFn {
    operation(move |d, a| {
        // if not e:GetHandler():IsRelateToEffect(e) then return end
        let h = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, h, a.e) {
            return;
        }
        // op(e,tp,eg,ep,ev,re,r,rp)
        op.call(d, &base_args(a));
    })
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousPosition(POS_FACEUP) and e:GetHandler():IsPreviousLocation(LOCATION_SZONE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, h, POS_FACEUP) && card::is_previous_location(d, h, LOCATION_SZONE)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FIRE_FIST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (tp, e) = (a.tp, a.e);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let (tp, e) = (a.tp, a.e);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
