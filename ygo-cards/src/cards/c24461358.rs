//! 朽ちた祭儀要録
//! Ragged Records of Rites
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 24461358;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 1 Ritual Spell in your hand or Deck, and add 1 monster that mentions it from your Deck to your hand
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
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a face-up Ritual Monster(s) you control leaves the field by card effect while this card is in your GY, and you control no face-up non-Ritual Monsters: You can add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfthcon)
    effect::set_condition(d, e2, Some(condition(selfthcon)));
    // e2:SetTarget(s.selfthtg)
    effect::set_target(d, e2, Some(cost_target(selfthtg)));
    // e2:SetOperation(s.selfthop)
    effect::set_operation(d, e2, Some(operation(selfthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.revealfilter(c,tp)`
fn revealfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualSpell() and not c:IsPublic() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    if !(card::is_ritual_spell(d, c) && !card::is_public(d, c)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsMonster() and c:ListsCode(code) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| revealfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local ritual_spell=sg:Filter(Card.IsRitualSpell,nil):GetFirst()
    let rs = group::filter(d, sg, &|d, x| card::is_ritual_spell(d, x), NO_EX);
    let ritual_spell = group::get_first(d, &rs);
    // return ritual_spell and sg:IsExists(s.thfilter,1,nil,ritual_spell:GetCode()),not ritual_spell
    match ritual_spell {
        Some(rsc) => {
            let code = card::get_code(d, rsc)[0];
            (group::is_exists(d, sg, &|d, x| thfilter(d, x, code), 1, NO_EX), false)
        }
        None => (false, true),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.revealfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,tp)
    let mut g = duel::get_matching_group(d, Some(&|d, x| revealfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // g:Merge(Duel.GetMatchingGroup(aux.AND(Card.IsMonster,Card.IsAbleToHand),tp,LOCATION_DECK,0,nil))
    let mons = duel::get_matching_group(d, Some(&|d, x| card::is_monster(d, x) && card::is_able_to_hand(d, x, None)), tp, LOCATION_DECK, 0, NO_EX);
    group::add_card(&mut g, &mons);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,2))
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 2)), None, None, false)
        .unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // local opp=1-tp
        let opp = 1 - tp;
        // local ritual_spell,monster=sg:Split(Card.IsRitualSpell,nil)
        let (ritual_spell, monster) = group::split(d, &sg, &|d, x| card::is_ritual_spell(d, x), NO_EX);
        // Duel.ConfirmCards(opp,ritual_spell)
        duel::confirm_cards(d, opp, ritual_spell);
        // Duel.SendtoHand(monster,nil,REASON_EFFECT)
        duel::send_to_hand(d, monster.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(opp,monster)
        duel::confirm_cards(d, opp, monster);
    }
}
/// `s.selfthconfilter(c,tp)`
fn selfthconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualMonster() and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsReason(REASON_EFFECT)
    card::is_ritual_monster(d, c)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.selfthcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.selfthconfilter,1,nil,tp)
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    if eg.contains(c) {
        return false;
    }
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, x| selfthconfilter(d, x, tp), 1, NO_EX)
}
/// `s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand()
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(aux.NOT(Card.IsRitualMonster)),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && !duel::is_existing_matching_card(
                d,
                Some(&aux::faceup_filter(|d, x| !card::is_ritual_monster(d, x))),
                tp,
                LOCATION_MZONE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfthop(e,tp,eg,ep,ev,re,r,rp)`
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
