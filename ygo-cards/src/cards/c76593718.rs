//! Greed Jar
use super::prelude::*;

pub const ID: u32 = 76593718;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Aply two effects in sequence when it is flipped
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct1=math.min(Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD),Duel.GetFieldGroupCount(tp,LOCATION_DECK,0))
    let ct1 = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD).min(duel::get_field_group_count(d, tp, LOCATION_DECK, 0));
    // local break_chk=false
    let mut break_chk = false;
    // Excavate cards from your Deck, add 1 to the hand and send the others to the GY
    // if ct1>0 then
    if ct1 > 0 {
        // break_chk=true
        break_chk = true;
        // local ac=Duel.AnnounceNumberRange(tp,1,ct1)
        let ac = duel::announce_number_range(d, tp, Some(1), Some(ct1 as u32), &[]).nil().0 as u32;
        // Duel.ConfirmDecktop(tp,ac)
        duel::confirm_decktop(d, tp, ac);
        // local g=Duel.GetDecktopGroup(tp,ac)
        let mut g = duel::get_decktop_group(d, tp, ac);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let sg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.DisableShuffleCheck(true)
            duel::disable_shuffle_check(d, true);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg.clone());
            // g:RemoveCard(sg:GetFirst())
            let first = group::get_first(d, &sg).nil();
            group::remove_card(&mut g, &CardSet::from_card(first));
        }
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
            duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
        }
    }
    // Send monsters from your Extra Deck to the GY
    // local ct2=Duel.GetMatchingGroupCount(Card.IsSummonLocation,tp,0,LOCATION_MZONE,nil,LOCATION_EXTRA)
    let ct2 = duel::get_matching_group_count(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), tp, 0, LOCATION_MZONE, NO_EX);
    // if ct2>0 then
    if ct2 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,ct2,nil)
        let tg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_EXTRA, 0, 1, ct2 as u16, false, NO_EX).nil();
        // if #tg>0 then
        if !tg.is_empty() {
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // Duel.SendtoGrave(tg,REASON_EFFECT)
            duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
        }
    }
}
