//! Newbee!
use super::prelude::*;

pub const ID: u32 = 10807219;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply any of these effects, in sequence
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_summon_location(d, c, LOCATION_HAND)
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttack(0) and c:IsDefense(0)
    card::is_attack(d, c, &[0]) && card::is_defense(d, c, &[0])
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_GRAVE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_INSECT) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_INSECT)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsFaceup() and c:IsAbleToDeck()
    card::is_trap(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     or Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local breakeffect=false
    let mut breakeffect = false;
    // local b1=Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(tdfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    // if b1 and (not b2 or Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    if b1 && (!b2 || duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
        // breakeffect=true
        breakeffect = true;
    }
    // if b2 and (not breakeffect or Duel.SelectYesNo(tp,aux.Stringid(id,3))) then
    if b2 && (!breakeffect || duel::select_yes_no(d, tp, aux::stringid(ID, 3))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(tdfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local seq=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 and Duel.SelectOption(tp,aux.Stringid(id,4),aux.Stringid(id,5)) or 0
            let seq = if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 {
                duel::select_option(d, tp, true, &[aux::stringid(ID, 4), aux::stringid(ID, 5)])
            } else {
                0
            };
            // if breakeffect then Duel.BreakEffect() end
            if breakeffect {
                duel::break_effect(d);
            }
            // Duel.SendtoDeck(g,nil,seq==0 and SEQ_DECKTOP or SEQ_DECKBOTTOM,REASON_EFFECT)
            let sq = if seq == 0 { SEQ_DECKTOP } else { SEQ_DECKBOTTOM };
            duel::send_to_deck(d, g, None, sq as i32, REASON_EFFECT, None);
        }
    }
}
