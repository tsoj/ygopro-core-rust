//! Harpie Lady Elegance
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 65664792;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HARPIE as u32], listed_names: &[CARD_HARPIE_LADY_SISTERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle 1 "Harpie Lady Sisters" into the Deck and Special Summon 3 "Harpie" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK|CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK | CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Harpie" monster from the Deck to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdfilter(c,e,tp,g)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsCode(CARD_HARPIE_LADY_SISTERS) and c:IsFaceup() and c:IsAbleToDeck()
    //     and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_TOFIELD)>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
    card::is_code(d, c, &[CARD_HARPIE_LADY_SISTERS])
        && card::is_faceup(d, c)
        && card::is_able_to_deck(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), None).0 >= 3
        && {
            let rs: aux::Rescon = &spcheck;
            matches!(aux::select_unselect_group(d, g, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
        }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_HARPIE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_HARPIE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcheck(sg,e,tp,mg)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetOriginalCodeRule)==#sg and sg:GetClassCount(Card.GetLocation)==#sg
    let n = sg.len();
    let by_code = group::get_class_count(d, sg, &|d, c| card::get_original_code_rule(d, c).first().copied().unwrap_or(0) as i64);
    let by_loc = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64);
    (by_code == n && by_loc == n, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,g)
        return duel::is_existing_matching_card(d, Some(&|d, c| tdfilter(d, c, e, tp, &g)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,g)
    let dg = duel::select_matching_card(d, tp, Some(&|d, c| tdfilter(d, c, e, tp, &g)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #dg>0 and Duel.SendtoDeck(dg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if !dg.is_empty() && duel::send_to_deck(d, dg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,1,tp,HINTMSG_SPSUMMON)
        let rs: aux::Rescon = &spcheck;
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Cannot Special Summon non-WIND monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_attribute(d, c, ATTRIBUTE_WIND))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and (rp==1-tp or (rp==tp and re:GetHandler():IsSetCard(SET_HARPIE))) and c:IsPreviousControler(tp)
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && (a.rp == 1 - tp || (a.rp == tp && {
            let rh = effect::get_handler(d, a.re.nil()).nil();
            card::is_set_card(d, rh, &[SET_HARPIE as u16])
        }))
        && card::is_previous_controler(d, c, tp)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_HARPIE) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_HARPIE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
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
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
