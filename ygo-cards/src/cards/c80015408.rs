//! Enneacraft Reverth
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 80015408;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ENNEACRAFT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle any number of "Enneacraft" Monster Cards from your hand and/or face-up field into the Deck, then draw cards equal to the number shuffled
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2a, LOCATION_GRAVE);
    // e2a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2a, Some(condition(opp_rp_cond)));
    // e2a:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2a, Some(cost::self_banish()));
    // e2a:SetTarget(s.efftg)
    effect::set_target(d, e2a, Some(cost_target(efftg)));
    // e2a:SetOperation(s.effop)
    effect::set_operation(d, e2a, Some(operation(effop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end`
fn opp_rp_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsMonsterCard() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //   and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16])
        && card::is_monster_card(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //   and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 0) && duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=g:Select(tp,1,#g,nil)
    let sg = group::select(d, &g, tp, 1, g.len() as u16, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local hg,fg=sg:Split(Card.IsLocation,nil,LOCATION_HAND)
    let (hg, fg) = group::split(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // if #hg>0 then Duel.ConfirmCards(1-tp,hg) end
    if !hg.is_empty() {
        duel::confirm_cards(d, 1 - tp, hg);
    }
    // if #fg>0 then Duel.HintSelection(fg) end
    if !fg.is_empty() {
        duel::hint_selection(d, fg, true);
    }
    // if Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and Duel.IsPlayerCanDraw(tp) then
    if duel::send_to_deck(d, sg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && duel::is_player_can_draw(d, tp, 0) {
        // local og=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_DECK)
        let mut og = duel::get_operated_group(d);
        group::match_(d, &mut og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // if #og>0 then
        if !og.is_empty() {
            // if og:IsExists(Card.IsControler,1,nil,tp) then Duel.ShuffleDeck(tp) end
            if group::is_exists(d, &og, &|d, c| card::is_controler(d, c, tp), 1, NO_EX) {
                duel::shuffle_deck(d, tp);
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,#og,REASON_EFFECT)
            duel::draw(d, tp, og.len() as u32, REASON_EFFECT);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsFacedown() and c:IsDefensePos() and c:IsCanChangePosition()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_facedown(d, c) && card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_POSITION)
        effect::set_category(d, a.e, CATEGORY_POSITION);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,POS_FACEUP_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, tp, POS_FACEUP_DEFENSE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Special Summon 1 "Enneacraft" monster from your hand in face-down Defense Position
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
        if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0 {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else if op == 2 {
        // Change 1 face-down Defense Position "Enneacraft" monster you control to face-up Defense Position
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.posfilter,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
            duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
}
