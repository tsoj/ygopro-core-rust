//! P.U.N.K. JAM Dragon Drive
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 28403802;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PSYCHIC),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, sc, st, Some(tp)))), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Search or send to the GY 1 "P.U.N.K." monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thgcon)
    effect::set_condition(d, e1, Some(condition(thgcon)));
    // e1:SetCost(Cost.PayLP(600))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(600))));
    // e1:SetTarget(s.thgtg)
    effect::set_target(d, e1, Some(cost_target(thgtg)));
    // e1:SetOperation(s.thgop)
    effect::set_operation(d, e1, Some(operation(thgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
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
/// `s.thgcon(e,tp,eg,ep,ev,re,r,rp)`
fn thgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() or (re and re:GetHandler():IsSetCard(SET_PUNK))
    let h = effect::get_handler(d, a.e).nil();
    if card::is_synchro_summoned(d, h) {
        return true;
    }
    match a.re {
        Some(re) => {
            let rh = effect::get_handler(d, re).nil();
            card::is_set_card(d, rh, &[SET_PUNK as u16])
        }
        None => false,
    }
}
/// `s.thgfilter(c)`
fn thgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(3) and c:IsRace(RACE_PSYCHIC) and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_level(d, c, &[3]) && card::is_race(d, c, RACE_PSYCHIC) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.thgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.thgop(e,tp,eg,ep,ev,re,r,rp)`
fn thgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.thgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&thgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // aux.ToHandOrElse(tc,tp)
    aux::to_hand_or_else(d, tc, a.tp, None, None, None);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ch=Duel.GetCurrentChain()-1
    let ch = duel::get_current_chain(d, false) - 1;
    // if ch<=0 then return false end
    if ch <= 0 {
        return false;
    }
    // local cplayer=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_CONTROLER)
    let cplayer = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_CONTROLER])[0].int();
    // local ceff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT)
    let ceff = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // return ep==1-tp and cplayer==tp and ceff:GetHandler():IsSetCard(SET_PUNK)
    if a.ep as i64 != 1 - a.tp as i64 || cplayer != a.tp as i64 {
        return false;
    }
    let ch_h = effect::get_handler(d, ceff).nil();
    card::is_set_card(d, ch_h, &[SET_PUNK as u16])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
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
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
