//! Azamina Mu Rcielago
//! 告死聖徒ルシエラーゴ
use super::prelude::*;

pub const ID: u32 = 73391962;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_AZAMINA, SET_SINFUL_SPOILS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Illusion monster + 1 LIGHT Spellcaster monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ILLUSION),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_ILLUSION, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp))),
        ],
    );
    // Monsters your opponent controls lose 500 ATK/DEF for each "Azamina" monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_AZAMINA),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)*-500 end)
    effect::set_value_fn(d, e1, effect_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Azamina" or "Sinful Spoils" card from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h)
    })));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add 1 "Sinful Spoils" Spell from your Deck to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e4, Some(condition(|_d, a| a.r & (REASON_BATTLE | REASON_EFFECT) > 0)));
    // e4:SetTarget(s.desthtg)
    effect::set_target(d, e4, Some(cost_target(desthtg)));
    // e4:SetOperation(s.desthop)
    effect::set_operation(d, e4, Some(operation(desthop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,fc,sumtype,tp) and c:IsRace(RACE_SPELLCASTER,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), st, Some(tp)) && card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp))
}
/// `function(e,c) return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_AZAMINA),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)*-500 end`
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_AZAMINA as u16])), p, LOCATION_MZONE, 0, NO_EX) as i64 * -500
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_AZAMINA,SET_SINFUL_SPOILS}) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_AZAMINA as u16, SET_SINFUL_SPOILS as u16]) && card::is_able_to_hand(d, c, None)
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
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoHand(g,tp,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), Some(tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.desthfilter(c)`
fn desthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SINFUL_SPOILS) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.desthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.desthop(e,tp,eg,ep,ev,re,r,rp)`
fn desthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&desthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
