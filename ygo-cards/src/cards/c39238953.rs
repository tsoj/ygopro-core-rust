//! Lullaby of Obedience
use super::prelude::*;

pub const ID: u32 = 39238953;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return (Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_DECK,1,nil) or Duel.IsPlayerCanSpecialSummon(tp))
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return (duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_DECK, 1, NO_EX) || duel::is_player_can_special_summon(d, tp, None))
            && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_MONSTER,OPCODE_ISTYPE,TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT,OPCODE_AND}
    // (only used by the next line: a constant here)
    let announce_filter = [TYPE_MONSTER as u64, OPCODE_ISTYPE, TYPE_EXTRA as u64, OPCODE_ISTYPE, OPCODE_NOT, OPCODE_AND];
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    crate::class::set_announce_filter(d, ID, &announce_filter);
    let ac = duel::announce_card(d, tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_DECK)
    let g = duel::get_field_group(d, tp, 0, LOCATION_DECK);
    // if #g<1 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_CONFIRM as u64);
    // local sg=g:FilterSelect(1-tp,Card.IsCode,1,1,nil,ac)
    let sg = group::filter_select(d, &g, 1 - tp, &|d, c| card::is_code(d, c, &[ac as u32]), 1, 1, false, NO_EX).nil();
    // local tc=sg:GetFirst()
    let tc = group::get_first(d, &sg);
    // if tc then
    if let Some(tc) = tc {
        // Duel.ConfirmCards(tp,sg)
        duel::confirm_cards(d, tp, &sg);
        // local b1=tc:IsAbleToHand()
        let b1 = card::is_able_to_hand(d, tc, None);
        // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and tc:IsCanBeSpecialSummoned(e,0,tp,true,false,POS_FACEUP_ATTACK,tp)
        let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, e, 0, tp, true, false, Some(POS_FACEUP_ATTACK as u8), Some(tp), None);
        // local sel=0
        let mut sel = 0;
        // if b1 and b2 then
        if b1 && b2 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_OPTION)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_OPTION as u64);
            // sel=Duel.SelectOption(1-tp,aux.Stringid(id,0),aux.Stringid(id,1))+1
            sel = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) + 1;
        // elseif b1 then
        } else if b1 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_OPTION)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_OPTION as u64);
            // sel=Duel.SelectOption(1-tp,aux.Stringid(id,0))+1
            sel = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 0)]) + 1;
        // elseif b2 then
        } else if b2 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_OPTION)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_OPTION as u64);
            // sel=Duel.SelectOption(1-tp,aux.Stringid(id,1))+2
            sel = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 1)]) + 2;
        }
        // if sel==1 then
        if sel == 1 {
            // Duel.SendtoHand(sg,tp,REASON_EFFECT)
            duel::send_to_hand(d, &sg, Some(tp), REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, &sg);
        // elseif sel==2 then
        } else if sel == 2 {
            // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP_ATTACK)
            duel::special_summon(d, &sg, 0, tp, tp, true, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
    // Duel.ShuffleDeck(1-tp)
    duel::shuffle_deck(d, 1 - tp);
}
