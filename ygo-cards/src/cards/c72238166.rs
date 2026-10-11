//! Node Ryzeal
use super::prelude::*;

pub const ID: u32 = 72238166;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| selfspcon(d, a))));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(|d, a| selfspop(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Ryzeal" monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.gyspcost)
    effect::set_cost(d, e2, Some(cost_target(gyspcost)));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_XYZ)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)
fn selfspop(d: &mut Duel, a: Args<'_>) {
    let tp = a.p(1);
    let c = a.c(8);
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Rank 4 Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not (c:IsType(TYPE_XYZ) and c:IsRank(4)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !(card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[4])))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard Check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not (c:IsOriginalType(TYPE_XYZ) and c:IsOriginalRank(4)) end)
    aux::add_temp_lizard_check(d, c, tp, Some(lua_fn(|d, a| lizard_filter(d, a))), None, None, None, None);
}
// function(e,c) return not (c:IsOriginalType(TYPE_XYZ) and c:IsOriginalRank(4)) end
fn lizard_filter(d: &mut Duel, a: Args<'_>) -> bool {
    let c = a.c(1);
    !(card::is_original_type(d, c, TYPE_XYZ) && card::is_original_rank(d, c, 4))
}
// function s.gyspcostfilter(c,tp)
fn gyspcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and Duel.GetMZoneCount(tp,c)>0
    card::is_able_to_grave_as_cost(d, c) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.gyspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn gyspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyspcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| gyspcostfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| gyspcostfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.gyspfilter(c,e,tp)
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_RYZEAL as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
// function s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.gyspfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && gyspfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==1 or Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let cost_chk = effect::get_label(d, a.e) == 1 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        return cost_chk && duel::is_existing_target(d, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
// function s.gyspop(e,tp,eg,ep,ev,re,r,rp)
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
        // tc:NegateEffects(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, None, false, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
