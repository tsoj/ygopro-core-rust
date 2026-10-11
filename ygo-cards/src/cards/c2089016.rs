//! Chronomaly Tuspa Rocket
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 2089016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce the ATK of a monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Grant a double attacking effect to a "Number" Xyz Monster that is summoned using this card as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.mtcon)
    effect::set_condition(d, e3, Some(condition(mtcon)));
    // e3:SetOperation(s.mtop)
    effect::set_operation(d, e3, Some(operation(mtop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.listed_series={SET_CHRONOMALY,SET_NUMBER}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY, SET_NUMBER], ..ScriptInfo::DEFAULT };

// function s.atkcostfilter(c)
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CHRONOMALY) and (c:IsLevelAbove(1) or c:IsRankAbove(1)) and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
        && (card::is_level_above(d, c, 1) || card::is_rank_above(d, c, 1))
        && card::is_able_to_grave_as_cost(d, c)
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcostfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let tc = duel::select_matching_card(d, a.tp, Some(&atkcostfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX)
        .nil();
    let tc = group::get_first(d, &tc).nil();
    // Duel.SendtoGrave(tc,REASON_COST)
    duel::send_to_grave(d, tc, REASON_COST, None, None);
    // local lvrnk=tc:HasLevel() and tc:GetLevel() or tc:GetRank()
    let lvrnk = if card::has_level(d, tc) { card::get_level(d, tc) as i64 } else { card::get_rank(d, tc) as i64 };
    // e:SetLabel(lvrnk)
    effect::set_label(d, a.e, &[lvrnk]);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let owner = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, owner);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel()*-200)
        let lbl = effect::get_label(d, a.e);
        effect::set_value(d, e1, lbl * -200);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.mtcon(e,tp,eg,ep,ev,re,r,rp)
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return r==REASON_XYZ and c:IsPreviousLocation(LOCATION_ONFIELD) and c:GetReasonCard():IsSetCard(SET_NUMBER)
    a.r == REASON_XYZ
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_set_card(d, card::get_reason_card(d, c).nil(), &[SET_NUMBER as u16])
}
// function s.mtop(e,tp,eg,ep,ev,re,r,rp)
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // Can make up to 2 attacks on monsters during each Battle Phase
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(3202)
    effect::set_description(d, e1, 3202);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // Becomes an Effect Monster if it wasn't one already
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
