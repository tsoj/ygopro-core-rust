//! GD Robo Orbital 7
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 91714772;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHOTON, SET_GALAXY, SET_GALAXY_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --You can Tribute this card from your hand or field; take 1 "Photon" monster and 1 "Galaxy" monster from your Deck, add 1 of them to your hand, and send the other to the GY, also you cannot Special Summon for the rest of this turn, except LIGHT monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.thtgtg)
    effect::set_target(d, e1, Some(cost_target(thtgtg)));
    // e1:SetOperation(s.thtgop)
    effect::set_operation(d, e1, Some(operation(thtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If a "Galaxy-Eyes" monster(s) is Special Summoned to your field: You can Special Summon this card from your GY, but banish it when it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
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
/// `s.thtgfilter(c)`
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_PHOTON,SET_GALAXY}) and c:IsMonster() and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16]) && card::is_monster(d, c) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_PHOTON) and sg:IsExists(Card.IsSetCard,1,nil,SET_GALAXY)
    //   and sg:IsExists(Card.IsAbleToHand,1,nil) and sg:IsExists(Card.IsAbleToGrave,1,nil)
    let res = group::is_exists(d, sg, &|d, x| card::is_set_card(d, x, &[SET_PHOTON as u16]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, x| card::is_set_card(d, x, &[SET_GALAXY as u16]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, x| card::is_able_to_hand(d, x, None), 1, NO_EX)
        && group::is_exists(d, sg, &|d, x| card::is_able_to_grave(d, x, None), 1, NO_EX);
    (res, false)
}
/// `s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thtgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thtgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thtgop(e,tp,eg,ep,ev,re,r,rp)`
fn thtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(s.thtgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thtgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,2))
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 2)), None, None, false).unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hg=sg:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let hg = group::filter_select(d, &sg, tp, &|d, x| card::is_able_to_hand(d, x, None), 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg.clone());
        // Duel.SendtoGrave(sg-hg,REASON_EFFECT)
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &hg);
        duel::send_to_grave(d, rest, REASON_EFFECT, None, None);
    }
    // --You cannot Special Summon for the rest of this turn, except LIGHT monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsAttributeExcept(ATTRIBUTE_LIGHT) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_attribute_except(d, c, ATTRIBUTE_LIGHT, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GALAXY_EYES) and c:IsControler(tp) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GALAXY_EYES as u16]) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| spconfilter(d, x, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
