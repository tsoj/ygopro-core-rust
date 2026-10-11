//! Defense of the Temple
//! 神殿の守護神
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 96687733;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TEMPLE_OF_THE_KINGS, 22082432], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 EARTH Fusion Monster from your Extra Deck, using monsters from your hand or field
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_EARTH),nil,s.fextra)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_EARTH))),
            extrafil: Some(Arc::new(fextra)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Dangers of the Divine" from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TEMPLE_OF_THE_KINGS),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(
        d,
        e2,
        Some(lua_fn(|d: &mut Duel, a: Args| {
            let tp = a.p(1);
            duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        })),
    );
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filterchk(c,tp)`
fn filterchk(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:ListsCode(CARD_TEMPLE_OF_THE_KINGS) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::lists_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS]) && card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    // if sg:IsExists(Card.IsControler,1,nil,1-tp) then
    if group::is_exists(d, &sg, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX) {
        // return sg:IsExists(s.filterchk,1,nil,tp)
        return group::is_exists(d, &sg, &|d, c| filterchk(d, c, tp), 1, NO_EX);
    }
    // return true
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if mg:IsExists(s.filterchk,1,nil,tp) then
    if group::is_exists(d, mg, &|d, c| filterchk(d, c, tp), 1, NO_EX) {
        // local eg=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup),tp,0,LOCATION_ONFIELD,nil)
        let eg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // if eg and #eg>0 then
        if !eg.is_empty() {
            // return eg,s.fcheck
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(22082432) and c:IsAbleToHand()
    card::is_code(d, c, &[22082432]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
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
        duel::confirm_cards(d, 1 - tp, g);
    }
}
