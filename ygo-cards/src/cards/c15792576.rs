//! Sky Scourge Cidhels
use super::prelude::*;

pub const ID: u32 = 15792576;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be special summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Apply an effect, based on the tributed monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp)return e:GetHandler():IsTributeSummoned()end)
    effect::set_condition(d, e1, Some(condition(tribute_summoned_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check the type/attribute of the monster for its tribute summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn tribute_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(s.filter1,1,nil) then
    let lo = effect::get_label_object(d, e).as_effect().nil();
    if group::is_exists(d, &g, &filter1, 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lo, &[1]);
    } else if group::is_exists(d, &g, &filter2, 1, NO_EX) {
        // e:GetLabelObject():SetLabel(2)
        effect::set_label(d, lo, &[2]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    // The value function returns nothing: the core reads nil as 0
    0
}
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FAIRY)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FAIRY)
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FIEND)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FIEND)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (s.filter1(c) or s.filter2(c)) and c:IsAbleToHand()
    (filter1(d, c) || filter2(d, c)) && card::is_able_to_hand(d, c, None)
}
fn tgfilter1(d: &mut Duel, c: CardId) -> bool {
    // return s.filter1(c) and c:IsAbleToGrave()
    filter1(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgfilter2(d: &mut Duel, c: CardId) -> bool {
    // return s.filter2(c) and c:IsAbleToGrave()
    filter2(d, c) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local var=e:GetLabel()
    let var = effect::get_label(d, a.e);
    // if chk==0 then
    if a.chk == Some(0) {
        // if var==1 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
        if var == 1 {
            return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
        } else if var == 2 {
            // return Duel.IsExistingMatchingCard(s.tgfilter1,...) or Duel.IsExistingMatchingCard(s.tgfilter2,...)
            return duel::is_existing_matching_card(d, Some(&tgfilter1), tp, LOCATION_DECK, 0, 1, NO_EX)
                || duel::is_existing_matching_card(d, Some(&tgfilter2), tp, LOCATION_DECK, 0, 1, NO_EX);
        }
    }
    // if var==1 then
    if var == 1 {
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else if var == 2 {
        // e:SetOperation(s.tgop)
        effect::set_operation(d, a.e, Some(operation(tgop)));
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn ctcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetAttribute)==#sg and sg:GetClassCount(Card.GetRace)==#sg
    let n = sg.len();
    let res = group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == n
        && group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == n;
    (res, false)
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1,g2=Duel.GetMatchingGroup(s.tgfilter1,tp,LOCATION_DECK,0,nil),Duel.GetMatchingGroup(s.tgfilter2,tp,LOCATION_DECK,0,nil)
    let mut g1 = duel::get_matching_group(d, Some(&tgfilter1), tp, LOCATION_DECK, 0, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&tgfilter2), tp, LOCATION_DECK, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local sg=aux.SelectUnselectGroup(g1,e,tp,1,2,s.ctcheck,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g1, Some(a.e), tp, Some(1), Some(2), Some(&ctcheck), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then Duel.SendtoGrave(sg,REASON_EFFECT) end
    if !sg.is_empty() {
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
