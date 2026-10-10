//! Hydralander Orbit
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 93953933;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate and add to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.excon)
    effect::set_condition(d, e1, Some(condition(excon)));
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.extg)
    effect::set_target(d, e1, Some(cost_target(extg)));
    // e1:SetOperation(s.exop)
    effect::set_operation(d, e1, Some(operation(exop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place on the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.excon(e,tp,eg,ep,ev,re,r,rp)`
fn excon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // return #g>=4 and g:GetClassCount(Card.GetCode)==#g
    g.len() >= 4 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) == g.len()
}
/// `s.extg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
        // if ct>0 then
        if ct > 0 {
            // local g=Duel.GetDecktopGroup(tp,ct)
            let g = duel::get_decktop_group(d, tp, ct as u32);
            // local result=g:FilterCount(Card.IsAbleToHand,nil)>0
            let result = group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
            // return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=ct and result
            return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= ct as i32 && result;
        }
        // return false
        return false;
    }
    // end (the target sets nothing when chk ~= 0)
    true
}
/// `s.exop(e,tp,eg,ep,ev,re,r,rp)`
fn exop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let mut ct = duel::get_matching_group_count(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, tp, ct as u32);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let g = duel::get_decktop_group(d, tp, ct as u32);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local g2=g:Filter(Card.IsMonster,nil)
    let g2 = group::filter(d, &g, &card::is_monster, NO_EX);
    // if #g2>=2 and g2:GetClassCount(Card.GetCode)==#g2 then
    if g2.len() >= 2 && group::get_class_count(d, &g2, &|d, c| card::get_code(d, c)[0] as i64) == g2.len() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g2:Select(tp,1,1,nil)
        let sg = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if sg:GetFirst():IsAbleToHand() then
        let first = group::get_first(d, &sg).nil();
        if card::is_able_to_hand(d, first, None) {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg.clone());
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        } else {
            // Duel.SendtoGrave(sg,REASON_RULE)
            duel::send_to_grave(d, sg, REASON_RULE, None, None);
        }
        // ct=ct-1
        ct -= 1;
    }
    // if ct>0 then Duel.SortDecktop(tp,tp,ct) end
    if ct > 0 {
        duel::sort_decktop(d, tp, tp, ct as u16);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // local h=e:GetHandler() (inline in the Lua)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
