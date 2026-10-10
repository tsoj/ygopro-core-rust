//! Pre-Preparation of Rites
use super::prelude::*;

pub const ID: u32 = 13048472;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Ritual Spell from your Deck to your hand, and add 1 Ritual Monster from your Deck or GY to your hand whose name is mentioned on that Ritual Spell. You can only activate 1 "Pre-Preparation of Rites" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spellfilter(c,tp)`
fn spellfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualSpell() and c:IsAbleToHand()
    // and Duel.IsExistingMatchingCard(s.monsterfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c)
    card::is_ritual_spell(d, c)
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| monsterfilter(d, cc, c)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.monsterfilter(c,ritual_spell)`
fn monsterfilter(d: &mut Duel, c: CardId, ritual_spell: CardId) -> bool {
    // return c:IsRitualMonster() and ritual_spell:ListsCode(c:GetCode()) and c:IsAbleToHand()
    if !card::is_ritual_monster(d, c) {
        return false;
    }
    let codes = card::get_code(d, c);
    card::lists_code(d, ritual_spell, &codes) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spellfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spellfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`: `ritual_spell and sg:IsExists(...), not ritual_spell`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local ritual_spell=sg:Filter(Card.IsRitualSpell,nil):GetFirst()
    let spells = group::filter(d, sg, &|d, c| card::is_ritual_spell(d, c), NO_EX);
    let ritual_spell = group::get_first(d, &spells);
    // return ritual_spell and sg:IsExists(s.monsterfilter,1,nil,ritual_spell),not ritual_spell
    let res = match ritual_spell {
        Some(rs) => group::is_exists(d, sg, &|d, c| monsterfilter(d, c, rs), 1, NO_EX),
        None => false,
    };
    (res, ritual_spell.is_none())
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spellfilter,tp,LOCATION_DECK,0,nil,tp)
    let mut g = duel::get_matching_group(d, Some(&|d, c| spellfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // g:Merge(Duel.GetMatchingGroup(aux.NecroValleyFilter(aux.AND(Card.IsRitualMonster,Card.IsAbleToHand)),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil))
    let nv = aux::necro_valley_filter(|d, c| card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None));
    let g2 = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    group::add_card(&mut g, &g2);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
