//! Life Extreme
use super::prelude::*;

pub const ID: u32 = 36148308;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[25165047], listed_series: &[SET_POWER_TOOL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.eqfilter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| eqfilter(d, x))), None, None, None, None, None, None);
    // Decrease battle target's ATK by 1500 during the Damage Calculation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetValue(-1500)
    effect::set_value(d, e1, -1500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e2, EVENT_CHANGE_POS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Life Stream Dragon"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_SZONE) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.eqfilter(c)
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsAttribute(ATTRIBUTE_EARTH)
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
// s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and e:GetHandler():GetEquipTarget():GetBattleTarget()
    if !duel::is_phase(d, PHASE_DAMAGE_CAL) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    card::get_battle_target(d, et).is_some()
}
// s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local bc=e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    let et = card::get_equip_target(d, h).nil();
    let bc = card::get_battle_target(d, et);
    // return c==bc and bc:IsControler(1-e:GetHandlerPlayer())
    match bc {
        Some(bc) => c == bc && card::is_controler(d, bc, 1 - effect::get_handler_player(d, e)),
        None => false,
    }
}
// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // local np=c:GetPosition()
    let np = card::get_position(d, c);
    // local pp=c:GetPreviousPosition()
    let pp = card::get_previous_position(d, c);
    // return not c:IsStatus(STATUS_CONTINUOUS_POS) and ((np<3 and pp>3) or (pp<3 and np>3))
    !card::is_status(d, c, STATUS_CONTINUOUS_POS) && ((np < 3 && pp > 3) || (pp < 3 && np > 3))
}
// s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
}
// s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_POWER_TOOL) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_set_card(d, c, &[SET_POWER_TOOL as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// s.spfilter(c,e,tp,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCode(25165047)
    //     and c:IsCanBeSpecialSummoned(e,TYPE_SYNCHRO,tp,false,false)
    duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, |m| m.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_code(d, c, &[25165047])
        && card::is_can_be_special_summoned(d, c, e, TYPE_SYNCHRO, tp, false, false, None, None, None)
}
// s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.costfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, a.tp, Some(&|d, c| costfilter(d, c, a.e, a.tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local g=Duel.SelectReleaseGroup(tp,s.costfilter,1,1,nil,e,tp)
    let g = duel::select_release_group(d, a.tp, Some(&|d, c| costfilter(d, c, a.e, a.tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
// function(e) return e:GetHandler():IsPreviousLocation(LOCATION_SZONE) end
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_SZONE)
}
