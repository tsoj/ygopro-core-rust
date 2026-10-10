//! Mysterious Guard
use super::prelude::*;

pub const ID: u32 = 1347977;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return to the top of the Deck or Deck & hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e) and (c:IsAbleToDeck() or c:IsAbleToHand())
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && (card::is_able_to_deck(d, c) || card::is_able_to_hand(d, c, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAbleToDeck,1,nil) and sg:IsExists(Card.IsAbleToHand,1,nil)
    let deck = group::is_exists(d, sg, &card::is_able_to_deck, 1, NO_EX);
    let hand = group::is_exists(d, sg, &|d, x| card::is_able_to_hand(d, x, None), 1, NO_EX);
    (deck && hand, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local dhg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let dhg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local b1=dhg:FilterCount(Card.IsAbleToDeck,nil)>0
    let b1 = group::filter_count(d, &dhg, &card::is_able_to_deck, NO_EX) > 0;
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,1,nil)
    //     and aux.SelectUnselectGroup(dhg,e,tp,2,2,s.rescon,0)
    let b2 = duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_WARRIOR))), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && matches!(aux::select_unselect_group(d, &dhg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return true;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // local tg=Group.CreateGroup()
    let tg = if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // tg=dhg:FilterSelect(tp,Card.IsAbleToDeck,1,1,nil)
        group::filter_select(d, &dhg, tp, &card::is_able_to_deck, 1, 1, false, NO_EX).nil()
    } else if op == Some(2) {
        // tg=aux.SelectUnselectGroup(dhg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
        let tg = aux::select_unselect_group(d, &dhg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg.clone()), 1, 0, 0);
        tg
    } else {
        CardSet::new()
    };
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 1, 0, 0);
    true
}
/// `s.selfilter(c,g)`
fn selfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:IsAbleToDeck() and g:IsExists(Card.IsAbleToHand,1,c)
    card::is_able_to_deck(d, c) && group::is_exists(d, g, &|d, x| card::is_able_to_hand(d, x, None), 1, c.into())
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==0 then return end
    if op == 0 {
        return;
    }
    if op == 1 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
        if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
            return;
        }
        // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    } else if op == 2 {
        // local tg=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
        let mut tg = duel::get_target_cards(d, e).nil();
        group::match_(d, &mut tg, &|d, c| card::is_faceup(d, c), NO_EX);
        // if #tg==0 then return end
        if tg.is_empty() {
            return;
        }
        // local dc=tg:GetFirst()
        // if #tg>1 then
        let dc = if tg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // dc=tg:FilterSelect(tp,s.selfilter,1,1,nil,tg):GetFirst()
            let sel = group::filter_select(d, &tg, tp, &|d, c| selfilter(d, c, &tg), 1, 1, false, NO_EX).nil();
            // if not dc then return end
            let Some(dc) = group::get_first(d, &sel) else {
                return;
            };
            // Duel.HintSelection(dc,true)
            duel::hint_selection(d, dc, true);
            dc
        } else {
            group::get_first(d, &tg).nil()
        };
        // tg=tg-dc
        group::remove_card(&mut tg, &CardSet::from_card(dc));
        // if Duel.SendtoDeck(dc,nil,SEQ_DECKTOP,REASON_EFFECT)>0 and dc:IsLocation(LOCATION_DECK) and #tg>0
        //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,1,nil) then
        if duel::send_to_deck(d, dc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
            && card::is_location(d, dc, LOCATION_DECK)
            && !tg.is_empty()
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_WARRIOR))), tp, LOCATION_MZONE, 0, 1, NO_EX)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
        }
    }
}
