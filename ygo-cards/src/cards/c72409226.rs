//! Materiactor Exagard
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 72409226;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 3 monsters
    // Xyz.AddProcedure(c,nil,3,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Special Summon 1 "Materiactor" monster from your Deck, OR add 1 "Materiactor" Spell/Trap from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.deckspthtg)
    effect::set_target(d, e1, Some(cost_target(deckspthtg)));
    // e1:SetOperation(s.deckspthop)
    effect::set_operation(d, e1, Some(operation(deckspthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add up to 2 cards attached to this card to your hand, including at least 1 "Materiactor" card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
    })));
    // e2:SetTarget(s.xyzthtg)
    effect::set_target(d, e2, Some(cost_target(xyzthtg)));
    // e2:SetOperation(s.xyzthop)
    effect::set_operation(d, e2, Some(operation(xyzthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.deckspthfilter(c,e,tp,mmz_chk)`
fn deckspthfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and ((c:IsMonster() and mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //   or (c:IsSpellTrap() and c:IsAbleToHand()))
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16])
        && ((card::is_monster(d, c) && mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)))
}
/// `s.deckspthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckspthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckspthfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| deckspthfilter(d, c, a.e, tp, mmz)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspthop(e,tp,eg,ep,ev,re,r,rp)`
fn deckspthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.deckspthfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0):GetFirst()
    let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| deckspthfilter(d, c, a.e, tp, mmz)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // if sc:IsMonster() then
    if card::is_monster(d, sc) {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // elseif sc:IsSpellTrap() then
    } else if card::is_spell_trap(d, sc) {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    }
}
/// `s.xyzthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyzthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local og=c:GetOverlayGroup():Filter(Card.IsAbleToHand,nil)
    let ov = card::get_overlay_group(d, c);
    let og = group::filter(d, &ov, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if chk==0 then return c:IsType(TYPE_XYZ) and og:IsExists(Card.IsSetCard,1,nil,SET_MATERIACTOR) end
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ) && group::is_exists(d, &og, &|d, c| card::is_set_card(d, c, &[SET_MATERIACTOR as u16]), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,og,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(og), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_MATERIACTOR)
    (group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_MATERIACTOR as u16]), 1, NO_EX), false)
}
/// `s.xyzthop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsType(TYPE_XYZ)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_type(d, c, TYPE_XYZ)) {
        return;
    }
    // local og=c:GetOverlayGroup():Filter(Card.IsAbleToHand,nil)
    let ov = card::get_overlay_group(d, c);
    let og = group::filter(d, &ov, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(og,e,tp,1,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &og, Some(a.e), tp, Some(1), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg>0 and Duel.SendtoHand(sg,nil,REASON_EFFECT)>0 then
    if !sg.is_empty() && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
        // if Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,TYPE_NORMAL)
        //   and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // local rthg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
            let rthg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
            // if #rthg==0 then return end
            if rthg.is_empty() {
                return;
            }
            // Duel.HintSelection(rthg)
            duel::hint_selection(d, rthg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(rthg,nil,REASON_EFFECT)
            duel::send_to_hand(d, rthg, None, REASON_EFFECT, None);
        }
    }
}
