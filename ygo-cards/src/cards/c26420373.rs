//! SR Passinglider
use super::prelude::*;

pub const ID: u32 = 26420373;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // scale change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.sccost)
    effect::set_cost(d, e1, Some(cost_target(sccost)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(lua_fn(hspcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot select battle target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetValue(s.atlimit)
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsType(TYPE_TUNER) and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.sccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetOriginalLevel())
    let first = group::get_first(d, &g).nil();
    let lvl = card::get_original_level(d, first);
    effect::set_label(d, a.e, &[lvl as i64]);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ct=e:GetLabel()
    let mut ct = effect::get_label(d, a.e);
    // local sel=0
    // if c:GetLeftScale()==1 then
    let sel = if card::get_left_scale(d, c) == 1 {
        // sel=Duel.SelectOption(tp,aux.Stringid(id,1))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
    } else {
        // sel=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    };
    // if sel==1 then
    if sel == 1 {
        // ct=-math.min(ct,c:GetLeftScale()-1)
        let ls = card::get_left_scale(d, c) as i64;
        ct = -std::cmp::min(ct, ls - 1);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_LSCALE)
    effect::set_code(d, e1, EFFECT_UPDATE_LSCALE);
    // e1:SetValue(ct)
    effect::set_value(d, e1, ct);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_RSCALE)
    effect::set_code(d, e2, EFFECT_UPDATE_RSCALE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,LOCATION_MZONE)==0
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_MZONE, LOCATION_MZONE) == 0 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and c:IsFaceup() and c:IsSetCard(SET_SPEEDROID)
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPEEDROID as u16])
}
