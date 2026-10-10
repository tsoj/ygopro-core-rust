//! Cycle of the World
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 32828635;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[46427957, 72426662, 8198712],
    fit_monster: &[46427957, 72426662],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon "Ruin, Queen of Oblivion" or "Demise, King of Armageddon"
    // Ritual.AddProcGreater({handler=c,filter=s.ritualfil,matfilter=s.forcedgroup})
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, e: EffectId, tp: u8| forcedgroup(d, x, e, tp)))),
            ..Default::default()
        },
    );
    // Add 1 "End of the World" from your Deck to your hand, then, you can add 1 "Ruin, Queen of Oblivion" or "Demise, King of Armageddon" from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(aux.exccon)
    effect::set_condition(d, e1, Some(aux::exccon()));
    // e1:SetCost(Cost.SelfToDeck)
    effect::set_cost(d, e1, Some(cost::self_to_deck()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ritualfil(c)`
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(46427957,72426662) and c:IsRitualMonster()
    card::is_code(d, c, &[46427957, 72426662]) && card::is_ritual_monster(d, c)
}
/// `s.forcedgroup(c,e,tp)`
fn forcedgroup(d: &mut Duel, c: CardId, _e: EffectId, _tp: u8) -> bool {
    // return c:IsLocation(LOCATION_ONFIELD)
    card::is_location(d, c, LOCATION_ONFIELD)
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(8198712) and c:IsAbleToHand()
    card::is_code(d, c, &[8198712]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gravethfilter(c)`
fn gravethfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(46427957,72426662) and c:IsAbleToHand()
    card::is_code(d, c, &[46427957, 72426662]) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // local mg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.gravethfilter),tp,LOCATION_GRAVE,0,nil)
        let nvf = aux::necro_valley_filter(gravethfilter);
        let mg = duel::get_matching_group(d, Some(&nvf), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #mg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !mg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=mg:Select(tp,1,1,nil)
            let sg = group::select(d, &mg, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
        }
    }
}
