//! Eva
use super::prelude::*;

pub const ID: u32 = 40177746;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is sent to the GY: You can banish up to 2 other LIGHT Fairy monsters from your field and/or GY; add the same number of Level 2 or lower LIGHT Fairy monsters with different names from your Deck to your hand, except "Eva"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FAIRY) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToRemoveAsCost()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FAIRY) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(2) and c:IsRace(RACE_FAIRY) and c:IsAttribute(ATTRIBUTE_LIGHT) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_level_below(d, c, 2) && card::is_race(d, c, RACE_FAIRY) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler())
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local dg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let dg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // local ct=math.min(2,dg:GetClassCount(Card.GetCode))
    let class_count = group::get_class_count(d, &dg, &|d, c| card::get_code(d, c)[0] as i64) as i64;
    let ct = std::cmp::min(2, class_count);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,ct,e:GetHandler())
    let rg = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, ct as u16, false, h.into()).nil();
    // local ct=Duel.Remove(rg,POS_FACEUP,REASON_COST)
    let ct = duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetParam(ct)
    duel::set_target_param(d, ct as u32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,ct,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, ct as u8, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if #g==0 or g:GetClassCount(Card.GetCode)<ct then return end
    if g.is_empty() || (group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64) < ct {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(ct as usize),
        Some(ct as usize),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_ATOHAND as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg==ct then
    if sg.len() as i64 == ct {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
