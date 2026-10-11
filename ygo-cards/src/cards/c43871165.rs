//! Jupredo the Shademachine Power Patron
use super::prelude::*;

pub const ID: u32 = 43871165;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[68231287], listed_series: &[SET_POWER_PATRON, SET_DOOMZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // During your Main Phase: You can destroy both this card and 1 "Power Patron" or "DoomZ" monster in your hand or face-up field, then you can destroy 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pdestg)
    effect::set_target(d, e1, Some(cost_target(pdestg)));
    // e1:SetOperation(s.pdesop)
    effect::set_operation(d, e1, Some(operation(pdesop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish (face-down) the top 3 cards of your Deck; destroy this card, and if you do, Special Summon 1 "Jupiter the Power Patron of Destruction" from your Extra Deck (this is treated as an Xyz Summon), then you can attach 1 card from your hand to it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.Replaceable(s.mdescost))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(mdescost), None)));
    // e2:SetTarget(s.mdestg)
    effect::set_target(d, e2, Some(cost_target(mdestg)));
    // e2:SetOperation(s.mdesop)
    effect::set_operation(d, e2, Some(operation(mdesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is added to your Extra Deck face-up: You can add 1 "Power Patron" or "DoomZ" Spell/Trap from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e3, EVENT_TO_DECK);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.pdesfilter(c)
fn pdesfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_POWER_PATRON,SET_DOOMZ}) and c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16, SET_DOOMZ as u16])
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
// function s.pdestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.pdesfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&pdesfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),2,tp,LOCATION_PZONE|LOCATION_MZONE|LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 2, tp, (LOCATION_PZONE | LOCATION_MZONE | LOCATION_HAND) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.pdesop(e,tp,eg,ep,ev,re,r,rp)
fn pdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.pdesfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,nil)
    let g1 = duel::select_matching_card(d, tp, Some(&pdesfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g1==0 then return end
    if g1.is_empty() {
        return;
    }
    // if g1:GetFirst():IsOnField() then Duel.HintSelection(g1) end
    let first = group::get_first(d, &g1).nil();
    if card::is_on_field(d, first) {
        duel::hint_selection(d, g1.clone(), true);
    }
    // if Duel.Destroy(g1+c,REASON_EFFECT)~=2 then return end
    let mut gc = g1.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    if duel::destroy(d, gc, REASON_EFFECT, None, None) != 2 {
        return;
    }
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #dg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if dg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=dg:Select(tp,1,1,nil)
    let g2 = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.HintSelection(g2)
        duel::hint_selection(d, g2.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(g2,REASON_EFFECT)
        duel::destroy(d, g2, REASON_EFFECT, None, None);
    }
}
// function s.mdescost(e,tp,eg,ep,ev,re,r,rp,chk)
fn mdescost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, tp, 3);
    // if chk==0 then return g:IsExists(Card.IsAbleToRemoveAsCost,3,nil,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return group::is_exists(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), 3, NO_EX);
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
// function s.jupiterspfilter(c,e,tp,hc)
fn jupiterspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: Option<CardId>) -> bool {
    // return c:IsCode(68231287) and Duel.GetLocationCountFromEx(tp,tp,hc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_code(d, c, &[68231287])
        && duel::get_location_count_from_ex(d, tp, Some(tp), hc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
// function s.mdestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.jupiterspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| jupiterspfilter(d, x, a.e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.mdesop(e,tp,eg,ep,ev,re,r,rp)
fn mdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.Destroy(c,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xc=Duel.SelectMatchingCard(tp,s.jupiterspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let xg = duel::select_matching_card(d, tp, Some(&|d, x| jupiterspfilter(d, x, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let xc = match group::get_first(d, &xg) {
        Some(xc) => xc,
        // if not xc then return end
        None => return,
    };
    // xc:SetMaterial(nil)
    card::set_material(d, xc, CardSet::new());
    // if Duel.SpecialSummon(xc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, xc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // xc:CompleteProcedure()
    card::complete_procedure(d, xc);
    // local g=Duel.GetMatchingGroup(Card.IsCanBeXyzMaterial,tp,LOCATION_HAND,0,nil,xc,tp,REASON_EFFECT)
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(xc), Some(tp), Some(REASON_EFFECT))),
        tp,
        LOCATION_HAND,
        0,
        NO_EX,
    );
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,4)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
        return;
    }
    // local attg=g:Select(tp,1,1,nil)
    let attg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #attg>0 then
    if !attg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(xc,attg)
        duel::overlay(d, xc, attg, false);
    }
}
// function s.thcon(e)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_POWER_PATRON,SET_DOOMZ}) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16, SET_DOOMZ as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
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
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
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
