//! Predaplant Longinephila
use super::prelude::*;

pub const ID: u32 = 44994712;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_POLYMERIZATION, ID],
    listed_series: &[SET_PREDAP as u32],
    counter_place_list: &[COUNTER_PREDATOR as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Predap" card from your Deck to your hand
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1a, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.thtg)
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    // e1a:SetOperation(s.thop)
    effect::set_operation(d, e1a, Some(operation(thop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Activate 1 of the following effects (Place Predator Counter, Set "Polymerization" from GY or banishment)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.predapthfilter(c)`
fn predapthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PREDAP) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PREDAP as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.predapthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&predapthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.predapthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&predapthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.predctfilter(c)`
fn predctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanAddCounter(COUNTER_PREDATOR,1)
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)
}
/// `s.polysetfilter(c)`
fn polysetfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsFaceup() and c:IsSSetable()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_faceup(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.efftg(e,tp,...,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(s.predctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&predctfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.polysetfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&polysetfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,2)},
    //   {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_COUNTER)
        effect::set_category(d, a.e, CATEGORY_COUNTER);
        // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,COUNTER_PREDATOR)
        duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, COUNTER_PREDATOR as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.predctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&predctfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if not tc then return end
        let Some(tc) = tc else {
            return;
        };
        // Duel.HintSelection(tc)
        duel::hint_selection(d, tc, true);
        // if tc:AddCounter(COUNTER_PREDATOR,1) and tc:IsLevelAbove(2) then
        if card::add_counter(d, tc, COUNTER_PREDATOR as u16, 1, false) && card::is_level_above(d, tc, 2) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetCondition(function(e) return e:GetHandler():HasCounter(COUNTER_PREDATOR) end)
            effect::set_condition(
                d,
                e1,
                Some(lua_fn(|d, a: Args| {
                    let h = effect::get_handler(d, a.e(0)).nil();
                    card::has_counter(d, h, COUNTER_PREDATOR as u16)
                })),
            );
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.polysetfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
        let sg = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(polysetfilter)),
            tp,
            LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.SSet(tp,sg)
            duel::s_set(d, tp, sg, None, true);
        }
    }
}
