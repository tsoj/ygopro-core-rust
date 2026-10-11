//! Mira Match
use super::prelude::*;

pub const ID: u32 = 44201739;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's card or effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.revfilter(c,tp)`
fn revfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not c:IsPublic() and Duel.IsExistingMatchingCard(Card.IsCode,tp,0,LOCATION_GRAVE,1,nil,c:GetCode())
    if card::is_public(d, c) {
        return false;
    }
    let codes = card::get_code(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_code(d, x, &codes)),
        tp,
        0,
        LOCATION_GRAVE,
        1,
        NO_EX,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| revfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && matches!(
                aux::select_unselect_group(
                    d,
                    &g,
                    Some(a.e),
                    tp,
                    Some(3),
                    Some(3),
                    Some(&aux::dncheck()),
                    0,
                    tp,
                    None,
                    None,
                    None,
                    false,
                ),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| revfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(3),
        Some(3),
        Some(&aux::dncheck()),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #rg==3 then
    if rg.len() == 3 {
        // Duel.ConfirmCards(1-tp,rg)
        duel::confirm_cards(d, 1 - tp, rg.clone());
        // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE,0,1)
            card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE), 0, 1, 0, 0);
        }
        // if rg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if group::is_exists(d, &rg, &|d, x| card::is_location(d, x, LOCATION_HAND), 1, NO_EX) {
            duel::shuffle_hand(d, tp);
        }
        // if rg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &rg, &|d, x| card::is_location(d, x, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, tp);
        }
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and not c:IsStatus(STATUS_BATTLE_DESTROYED) and c:HasFlagEffect(id) and Duel.IsChainNegatable(ev)
    a.rp == 1 - a.tp
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && card::has_flag_effect(d, c, ID, None)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.tgfilter(c,codes)`
fn tgfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsAbleToGrave() and c:IsCode(codes)
    card::is_able_to_grave(d, c, None) && card::is_code(d, c, codes)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local code1,code2=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
    let code1 = info[0].int() as u32;
    let code2 = info[1].int() as u32;
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,{code1,code2}) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| tgfilter(d, x, &[code1, code2])),
            tp,
            LOCATION_DECK | LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        );
    }
    // e:SetLabel(code1,code2)
    effect::set_label(d, a.e, &[code1 as i64, code2 as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local code1,code2=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let code1 = labels.first().copied().unwrap_or(0) as u32;
    let code2 = labels.get(1).copied().unwrap_or(0) as u32;
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,{code1,code2})
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| tgfilter(d, x, &[code1, code2])),
        tp,
        LOCATION_DECK | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_GRAVE) then
    if !g.is_empty()
        && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0
        && {
            let first = group::get_first(d, &g).nil();
            card::is_location(d, first, LOCATION_GRAVE)
        }
    {
        // Duel.NegateActivation(ev)
        duel::negate_activation(d, a.ev as u8);
    }
}
