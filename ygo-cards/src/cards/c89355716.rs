//! The Weather Sunny Canvas
use super::prelude::*;

pub const ID: u32 = 89355716;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // effect gain
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.announcecost)
    effect::set_cost(d, e2, Some(cost_target(announcecost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.eftg)
    effect::set_target(d, e3, Some(effect_card_fn(eftg)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eftg(e,c)`
fn eftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local g=e:GetHandler():GetColumnGroup(1,1)
    let h = effect::get_handler(d, e).nil();
    let g = card::get_column_group(d, h, 1, 1);
    // return c:IsType(TYPE_EFFECT) and c:IsSetCard(SET_THE_WEATHER) and c:GetSequence()<5 and g:IsContains(c)
    card::is_type(d, c, TYPE_EFFECT) && card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::get_sequence(d, c) < 5 && group::is_contains(&g, c)
}
/// `s.spcfilter(c,e,tp,ft)`
fn spcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsReleasableByEffect() and (ft>0 or c:GetSequence()<5)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,c:GetCode())
    if !card::is_releasable_by_effect(d, c, None, None) || !(ft > 0 || card::get_sequence(d, c) < 5) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_THE_WEATHER) and not c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_THE_WEATHER as u16])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.announcecost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn announcecost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk) end
    if a.chk == Some(0) {
        return bfgcost_call(d, a, 0);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    bfgcost_call(d, a, 1);
    true
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called with the arguments of the current cost
fn bfgcost_call(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    aux::bfgcost().call(d, &args).truthy()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.spcfilter(chkc,e,tp,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && spcfilter(d, chkc, a.e, tp, ft);
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.spcfilter,tp,LOCATION_MZONE,0,1,e:GetHandler(),e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spcfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // Duel.SelectTarget(tp,s.spcfilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler(),e,tp,ft)
    duel::select_target(d, tp, Some(&|d, c| spcfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsControler(tp) and tc:IsRelateToEffect(e) and Duel.Release(tc,REASON_EFFECT)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_controler(d, tc, tp)
        && card::is_relate_to_effect(d, tc, a.e)
        && duel::release(d, tc, REASON_EFFECT, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,tc:GetCode())
        let code = card::get_code(d, tc)[0];
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp, code))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
