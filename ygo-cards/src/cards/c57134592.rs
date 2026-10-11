//! Salamangreat Raging Phoenix
use super::prelude::*;

pub const ID: u32 = 57134592;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[57134592], listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // aux.EnableCheckReincarnation(c)
    aux::enable_check_reincarnation(d, c);
    // --Link Summon procedure
    // Link.AddProcedure(c,s.matfilter,2)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 2, None, None, None, None);
    // --Add 1 "Salamangreat" card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon itself
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,scard,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT,scard,sumtype,tp) and c:IsAttribute(ATTRIBUTE_FIRE,scard,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, sc, st, Some(tp))
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReincarnationSummoned() and c:IsLinkSummoned()
    card::is_reincarnation_summoned(d, c) && card::is_link_summoned(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,c)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and (c:GetPreviousAttributeOnField()&ATTRIBUTE_FIRE)==ATTRIBUTE_FIRE
    //   and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //   and c:IsCanBeEffectTarget(e) and c:GetAttack()>0 and not c:IsType(TYPE_TOKEN)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_FIRE) == ATTRIBUTE_FIRE
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::get_attack(d, c) > 0
        && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.cfilter,1,nil,e,tp)
    !eg.contains(h) && group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return eg.contains(chkc) && cfilter(d, chkc, a.e, tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and eg:IsExists(s.cfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && group::is_exists(d, &eg, &|d, x| cfilter(d, x, a.e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    // local tg=eg:FilterSelect(tp,s.cfilter,1,1,nil,e,tp)
    let tg = group::filter_select(d, &eg, tp, &|d, x| cfilter(d, x, a.e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,tg:GetFirst():GetAttack())
    let first = group::get_first(d, &tg).nil();
    let atk = card::get_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, tp, atk);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // --Increase ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
