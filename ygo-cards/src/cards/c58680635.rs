//! Electrode Beast Anion
use super::prelude::*;

pub const ID: u32 = 58680635;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[20802187], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make this card become Level 4 and search 1 "Spirit Converter"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_LVCHANGE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send two cards to the GY and draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(20802187) and c:IsAbleToHand()
    card::is_code(d, c, &[20802187]) && card::is_able_to_hand(d, c, None)
}
/// `s.lvtg(e,tp,...,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsLevel(4) end
    if a.chk == Some(0) {
        return !card::is_level(d, c, &[4]);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,c,1,tp,math.abs(c:GetLevel()-4))
    let lv = (card::get_level(d, c) as i32 - 4).abs();
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 1, tp, lv);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp,c)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and not c:IsLevel(4) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && !card::is_level(d, c, &[4]) {
        // Change its Level to 4
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(4)
        effect::set_value(d, e1, 4);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // if not c:IsLevel(4) then return end
        if !card::is_level(d, c, &[4]) {
            return;
        }
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK, LOCATION_GRAVE, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc~=c and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && chkc != c && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return c:IsAbleToGrave() and Duel.IsExistingTarget(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None)
            && duel::is_existing_target(d, Some(&|d, cc| card::is_able_to_grave(d, cc, None)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,LOCATION_ONFIELD,0,1,1,c)
    let g = duel::select_target(d, tp, Some(&|d, cc| card::is_able_to_grave(d, cc, None)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g+c,2,tp,0)
    let mut gc = g.clone().unwrap_or_default();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(gc), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    // if Duel.SendtoGrave(Group.CreateGroup(c,tc),REASON_EFFECT)==2
    //     and Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==2 then
    let mut gg = CardSet::from_card(c);
    group::add_card(&mut gg, &CardSet::from_card(tc));
    if duel::send_to_grave(d, gg, REASON_EFFECT, None, None) == 2
        && group::filter_count(d, &duel::get_operated_group(d), &|d, cc| card::is_location(d, cc, LOCATION_GRAVE), NO_EX) == 2
    {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
