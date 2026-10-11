//! Theorealize
use super::prelude::*;

pub const ID: u32 = 48739627;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MEDIUS_THE_PURE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Look at the top card of your opponent's Deck and place it on either the top or the bottom of the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Change the Monster Type and Attribute of 1 face-up monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
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
    // e2:SetTarget(s.chngtg)
    effect::set_target(d, e2, Some(cost_target(chngtg)));
    // e2:SetOperation(s.chngop)
    effect::set_operation(d, e2, Some(operation(chngop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_MEDIUS_THE_PURE) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[CARD_MEDIUS_THE_PURE]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,0)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, 0);
        // local ac=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
        let ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
        // if ac==1 then Duel.MoveSequence(g:GetFirst(),1) end
        if ac == 1 {
            let tc = group::get_first(d, &g).nil();
            duel::move_sequence(d, tc, 1, None);
        }
    }
}
/// `s.chngtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local race=Duel.AnnounceRace(tp,1,RACE_ALL)
    let race = duel::announce_race(d, a.tp, 1, RACE_ALL);
    // local attr=tc:IsRaceExcept(race) and Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL) or tc:AnnounceAnotherAttribute(tp)
    // (a number from AnnounceAttribute is always true in Lua, so this is an if/else)
    let attr = if card::is_race_except(d, tc, race, None, 0, None) {
        duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL)
    } else {
        card::announce_another_attribute(d, tc, a.tp)
    };
    // e:SetLabel(race,attr)
    effect::set_label(d, a.e, &[race as i64, attr as i64]);
    true
}
/// `s.chngop(e,tp,eg,ep,ev,re,r,rp)`
fn chngop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if (not tc:IsFaceup() and tc:IsRelateToEffect(e)) then return end
    if !card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local race,attr=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels[0] as u64;
    let attr = labels[1] as u32;
    // if tc:IsRaceExcept(race) then
    if card::is_race_except(d, tc, race, None, 0, None) {
        // --Change its Monster Type
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(race)
        effect::set_value(d, e1, race as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // if tc:IsAttributeExcept(attr) then
    if card::is_attribute_except(d, tc, attr, None, 0, None) {
        // --Change its Attribute
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
        // e2:SetValue(attr)
        effect::set_value(d, e2, attr as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
