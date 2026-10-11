//! Cyberdark Chimera
//! サイバー・ダーク・キメラ
use super::prelude::*;

pub const ID: u32 = 5370235;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[37630732], listed_series: &[SET_CYBER, SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Power Bond" from your Deck to your hand
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 "Cyberdark" monster from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsDiscardable),tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,aux.AND(Card.IsSpellTrap,Card.IsDiscardable),1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_spell_trap(d, c) && card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(37630732) and c:IsAbleToHand()
    card::is_code(d, c, &[37630732]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
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
        duel::confirm_cards(d, 1 - tp, g.clone());
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Can also banish monsters from your GY as material once when you Fusion Summon this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_FUSION_MATERIAL);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTargetRange(LOCATION_GRAVE,0)
    effect::set_target_range(d, e1, LOCATION_GRAVE, 0);
    // e1:SetTarget(function(e,c) return c:IsAbleToRemove() and c:IsMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_able_to_remove(d, c, None, None, None) && card::is_monster(d, c))));
    // e1:SetOperation(Fusion.BanishMaterial)
    effect::set_operation(d, e1, Some(fusion::banish_material()));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // Can only use Dragon or Machine "Cyber" monsters as Fusion Material this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetTargetRange(0xff,0xff)
    effect::set_target_range(d, e2, 0xff, 0xff);
    // e2:SetTarget(function(e,c) return not (c:IsSetCard(SET_CYBER) and c:IsRace(RACE_DRAGON|RACE_MACHINE)) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !(card::is_set_card(d, c, &[SET_CYBER as u16]) && card::is_race(d, c, RACE_DRAGON | RACE_MACHINE)))));
    // e2:SetValue(function(e,c) return c and c:IsControler(e:GetHandlerPlayer()) end)
    effect::set_value_fn(
        d,
        e2,
        lua_fn(|d, a: Args| -> Value {
            let c = a.card(1);
            match c {
                None => Value::Nil,
                Some(c) => {
                    let p = effect::get_handler_player(d, a.e(0));
                    Value::Bool(card::is_controler(d, c, p))
                }
            }
        }),
    );
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CYBERDARK) and c:IsMonster() and c:IsAbleToGrave()
    //     and not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,c:GetCode())
    card::is_set_card(d, c, &[SET_CYBERDARK as u16])
        && card::is_monster(d, c)
        && card::is_able_to_grave(d, c, None)
        && !{
            let codes = card::get_code(d, c);
            duel::is_existing_matching_card(d, Some(&|d, cc| card::is_code(d, cc, &codes)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
