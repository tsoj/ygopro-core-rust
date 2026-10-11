//! Mitsurugi Magatama
use super::prelude::*;

pub const ID: u32 = 76725398;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MITSURUGI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ritparams`
fn ritparams() -> ritual::RitualProc {
    ritual::RitualProc {
        // lvtype=RITPROC_GREATER
        lvtype: RITPROC_GREATER,
        // filter=aux.FilterBoolFunction(Card.IsSetCard,SET_MITSURUGI)
        filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MITSURUGI as u16]))),
        // matfilter=aux.FilterBoolFunction(Card.IsLocation,LOCATION_MZONE)
        matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| card::is_location(d, x, LOCATION_MZONE)))),
        ..Default::default()
    }
}
/// `s.rescon(sg,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, tp: u8, _mg: &CardSet) -> (bool, bool) {
    // return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,sg+sg:GetFirst():GetEquipGroup())
    let first = group::get_first(d, sg).nil();
    let eq = card::get_equip_group(d, first);
    let ex = group::union(sg, &eq);
    (duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, (&ex).into()), false)
}
/// `e,tp,eg,ep,ev,re,r,rp` followed by `chk`, as the core passes them to a Ritual target
fn eff_values_chk(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}
/// `e,tp,eg,ep,ev,re,r,rp` as the core passes them to an operation
fn eff_values(a: &EffArgs) -> Vec<Value> {
    eff_values_chk(a, 0)[..8].to_vec()
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // local b1=Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,false,s.rescon,nil,RACE_REPTILE)
    let b1 = duel::check_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_REPTILE)), 1, 1, false, Some(&rescon), NO_EX);
    // local b2=Ritual.Target(s.ritparams)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = ritual::target(d, ritparams()).call(d, &eff_values_chk(a, 0)).truthy();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // local rg=Duel.SelectReleaseGroupCost(tp,Card.IsRace,1,1,false,s.rescon,nil,RACE_REPTILE)
        let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_REPTILE)), 1, 1, false, Some(&rescon), NO_EX);
        // Duel.Release(rg,REASON_COST)
        duel::release(d, rg, REASON_COST, None);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local tg=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,1,nil)
        let tg = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, a.tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, a.tp, LOCATION_MZONE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Destroy 1 card your opponent controls
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Ritual Summon1 "Mitsurugi" Ritual Monster from your hand, by Tributing monsters you control
        // Ritual.Operation(s.ritparams)(e,tp,eg,ep,ev,re,r,rp)
        ritual::operation(d, ritparams()).call(d, &eff_values(a));
    }
}
