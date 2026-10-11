//! Hojo the Vaylantz Warrior
use super::prelude::*;

pub const ID: u32 = 88919365;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_VALIANTS_SHINRABANSHO],
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local fusparams={aux.FilterBoolFunction(Card.IsSetCard,SET_VAYLANTZ),nil,s.fextra}
    let fusparams = || fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_VAYLANTZ as u16]))),
        extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet| fextra(d, tp, mg))),
        ..Default::default()
    };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.fuscon)
    effect::set_condition(d, e3, Some(condition(fuscon)));
    // e3:SetTarget(Fusion.SummonEffTG(table.unpack(fusparams)))
    let tg = fusion::summon_eff_tg(d, fusparams());
    effect::set_target(d, e3, Some(tg));
    // e3:SetOperation(Fusion.SummonEffOP(table.unpack(fusparams)))
    let op = fusion::summon_eff_op(d, fusparams());
    effect::set_operation(d, e3, Some(op));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and c:IsAttribute(ATTRIBUTE_WATER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_VALIANTS_SHINRABANSHO,PLAYER_ALL,LOCATION_FZONE)
    //   or Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_environment(d, CARD_VALIANTS_SHINRABANSHO, Some(PLAYER_ALL as u8), Some(LOCATION_FZONE))
        || duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetSequence()<5 and c:IsAbleToHand()
    card::is_faceup(d, c) && card::get_sequence(d, c) < 5 && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.rthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && rthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rthfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rthfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_PZONE,0,nil)
    (Some(duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_PZONE, 0, NO_EX)), None)
}
/// `s.fuscon(e,tp,eg,ep,ev,re,r,rp)`
fn fuscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return not Duel.IsPhase(PHASE_DAMAGE) and c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    !duel::is_phase(d, PHASE_DAMAGE) && card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
