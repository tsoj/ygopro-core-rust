//! Tenyi Spirit - Shthana
use super::prelude::*;

pub const ID: u32 = 24557335;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon a non-Effect monster that was destroyed, then destroy 1 monster the opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,0,1,nil) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT))), a.tp, LOCATION_MZONE as u32, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousControler(tp) and c:IsPreviousPosition(POS_FACEUP)
    //     and not c:IsPreviousTypeOnField(TYPE_EFFECT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsCanBeEffectTarget(e)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP)
        && !card::is_previous_type_on_field(d, c, TYPE_EFFECT)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return spfilter(d, chkc, a.e, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and eg:IsExists(s.spfilter,1,nil,e,tp)
    //     and ((not eg:IsContains(c)) or c:IsLocation(LOCATION_HAND)) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d, c| spfilter(d, c, e, tp), 1, NO_EX)
            && (!group::is_contains(&eg, c) || card::is_location(d, c, LOCATION_HAND));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=eg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, a.tp, &|d, c| spfilter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let tc = tc.nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE as u32, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE as u32, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
