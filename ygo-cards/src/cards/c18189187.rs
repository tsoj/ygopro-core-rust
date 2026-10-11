//! Neiroy, the Dream Mirror Disciple
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 18189187;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DREAM_MIRROR],
    listed_names: &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
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
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return Spell/Trap to the hand + Draw and Shuffle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_DRAW + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.cond2)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.tg2)
    effect::set_target(d, e2, Some(cost_target(tg2)));
    // e2:SetOperation(s.op2)
    effect::set_operation(d, e2, Some(operation(op2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DREAM_MIRROR),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d: &mut Duel, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16])),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // if not c:IsAttribute(ATTRIBUTE_DARK) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !card::is_attribute(d, c, ATTRIBUTE_DARK) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
            effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetValue(ATTRIBUTE_DARK)
            effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.cond2(e,tp,eg,ep,ev,re,r,rp)`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    match a.re {
        None => false,
        Some(re) => {
            // return re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_DREAM_MIRROR)
            effect::is_monster_effect(d, re) && {
                let h = effect::get_handler(d, re).nil();
                card::is_set_card(d, h, &[SET_DREAM_MIRROR as u16])
            }
        }
    }
}
/// `s.tg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_SZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_SZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.ffilter(c,code)`
fn ffilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(code)
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.op2(e,tp,eg,ep,ev,re,r,rp)`
fn op2(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsExistingMatchingCard(s.ffilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,CARD_DREAM_MIRROR_JOY)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,0,LOCATION_ONFIELD,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c| ffilter(d, c, CARD_DREAM_MIRROR_JOY)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
    // if Duel.IsExistingMatchingCard(s.ffilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,CARD_DREAM_MIRROR_TERROR) and Duel.IsPlayerCanDraw(tp,1) then
    if duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c| ffilter(d, c, CARD_DREAM_MIRROR_TERROR)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::is_player_can_draw(d, a.tp, 1)
    {
        // local ct=Duel.Draw(tp,1,REASON_EFFECT)
        let ct = duel::draw(d, a.tp, 1, REASON_EFFECT);
        // if ct~=0 then
        if ct != 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
            // local tdg=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_HAND,0,1,1,nil)
            let tdg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(tdg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tdg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
