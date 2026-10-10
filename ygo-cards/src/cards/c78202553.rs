//! Subterror Behemoth Stalagmo
use super::prelude::*;

pub const ID: u32 = 78202553;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUBTERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e2, EVENT_CHANGE_POS);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // turn set
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_SUBTERROR) and c:IsDiscardable()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SUBTERROR as u16]) && card::is_discardable(d, c, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX) && duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.DiscardHand(tp,s.tgfilter,1,1,REASON_EFFECT|REASON_DISCARD)~=0 then
    if duel::discard_hand(d, a.tp, Some(&tgfilter), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) != 0 {
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsFacedown() and c:IsControler(tp)
    card::is_previous_position(d, c, POS_FACEUP) && card::is_facedown(d, c) && card::is_controler(d, c, tp)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    //     and not Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, tp), 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //     and not Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
            && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, 0, 0);
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
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, CardSet::from_card(c), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
