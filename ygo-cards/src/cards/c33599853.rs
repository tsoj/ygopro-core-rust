//! Light and Darkness Ritual
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 33599853;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44001993, 70405001, CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon 1 "Black Chaos the Dark Chaos Magician" or "Black Luster Soldier - Soldier of Light and Darkness" from your hand, by Tributing monsters from your hand or field, and/or banishing monsters from your GY, whose total Levels equal or exceed its Level
    // Ritual.AddProcGreater{
    //     handler=c,
    //     filter=function(c) return c:IsCode(44001993,70405001) end,
    //     extrafil=function(e,tp) return Duel.GetMatchingGroup(aux.AND(Card.HasLevel,Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil) end,
    //     extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE) end,
    //     desc=aux.Stringid(id,0)
    // }
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_code(d, c, &[44001993, 70405001]))),
            extrafil: Some(std::sync::Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extratg: Some(cost_target(extratg)),
            desc: Some(aux::stringid(ID, 0)),
            ..Default::default()
        },
    );
    // If this card is in your GY: You can add both this card and 1 card that mentions "Ritual of Light and Darkness" from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `extrafil=function(e,tp) return Duel.GetMatchingGroup(aux.AND(Card.HasLevel,Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil) end`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    Some(duel::get_matching_group(d, Some(&|d, c| card::has_level(d, c) && card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_GRAVE, 0, NO_EX))
}
/// `extratg=function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_LIGHT_AND_DARKNESS_RITUAL) and c:IsAbleToHand()
    card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL as u32]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // if #g>0 then
    if !g.is_empty() {
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
