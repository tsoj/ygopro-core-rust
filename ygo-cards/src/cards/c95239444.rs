//! Orea, the Sylvan High Arbiter
use super::prelude::*;

pub const ID: u32 = 95239444;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // Sort cards from the top of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.stcost)
    effect::set_cost(d, e1, Some(cost_target(stcost)));
    // e1:SetOperation(s.stop)
    effect::set_operation(d, e1, Some(operation(stop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate up to 3 cards from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,lv)`
fn cfilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsLevelBelow(lv) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_PLANT)
        && card::is_level_below(d, c, lv as u32)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.stcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn stcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) as i64;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,nil,ct) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, ct)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,nil,ct)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, ct)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetLevel())
    let tc = group::get_first(d, &g).nil();
    let lv = card::get_level(d, tc) as i64;
    effect::set_label(d, a.e, &[lv]);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.stop(e,tp,eg,ep,ev,re,r,rp)`
fn stop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=ct then
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) as i64 >= ct {
        // Duel.SortDecktop(tp,tp,ct)
        duel::sort_decktop(d, a.tp, a.tp, ct as u16);
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1)
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_discard_deck(d, a.tp, 1)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // local ct=math.min(3,Duel.GetFieldGroupCount(tp,LOCATION_DECK,0))
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0).min(3);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.Hint(HINTMSG_NUMBER,tp,HINT_NUMBER)
    duel::hint(d, HINTMSG_NUMBER as u8, a.tp, HINT_NUMBER as u64);
    // local ac=Duel.AnnounceNumberRange(tp,1,ct)
    let mut ac = duel::announce_number_range(d, a.tp, Some(1), Some(ct as u32), &[]).nil().0 as i64;
    // Duel.ConfirmDecktop(tp,ac)
    duel::confirm_decktop(d, a.tp, ac as u32);
    // local g=Duel.GetDecktopGroup(tp,ac)
    let g = duel::get_decktop_group(d, a.tp, ac as u32);
    // local sg=g:Filter(Card.IsRace,nil,RACE_PLANT)
    let sg = group::filter(d, &g, &|d, c| card::is_race(d, c, RACE_PLANT), NO_EX);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if Duel.SendtoGrave(sg,REASON_EFFECT+REASON_EXCAVATE)~=0 then
    if duel::send_to_grave(d, sg.clone(), REASON_EFFECT + REASON_EXCAVATE, None, None) != 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
        let og = duel::get_operated_group(d);
        let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
            // local tg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,#sg,c)
            let tg = duel::select_matching_card(
                d,
                a.tp,
                Some(&|d, x| card::is_able_to_hand(d, x, None)),
                a.tp,
                LOCATION_ONFIELD,
                LOCATION_ONFIELD,
                1,
                sg.len() as u16,
                false,
                c.into(),
            )
            .nil();
            // if #tg>0 then
            if !tg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.DisableShuffleCheck(false)
                duel::disable_shuffle_check(d, false);
                // Duel.SendtoHand(tg,nil,REASON_EFFECT)
                duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
            }
        }
    }
    // ac=ac-#sg
    ac -= sg.len() as i64;
    // if ac>0 then
    if ac > 0 {
        // Duel.SortDecktop(tp,tp,ac)
        duel::sort_decktop(d, a.tp, a.tp, ac as u16);
        // for i=1,ac do
        for _ in 0..ac {
            // local mg=Duel.GetDecktopGroup(tp,1)
            let mg = duel::get_decktop_group(d, a.tp, 1);
            // Duel.MoveSequence(mg:GetFirst(),1)
            let tc = group::get_first(d, &mg).nil();
            duel::move_sequence(d, tc, 1, None);
        }
    }
}
