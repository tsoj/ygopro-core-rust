//! Raise Moon Hope Squeeze - Jackpot
use super::prelude::*;

pub const ID: u32 = 100458023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Your opponent's monsters cannot target monsters for attacks, except this one
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c~=e:GetHandler() end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| c != effect::get_handler(d, e).nil()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Xyz Summoned, or a monster(s) is Special Summoned from the hand: You can detach any number of materials from this card; place that many cards your opponent controls on the bottom of the Deck in any order. You can only use this effect of "Raise Moon Hope Squeeze - Jackpot" once per turn
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2a, CATEGORY_TODECK);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e2a, Some(condition(e2a_cond)));
    // e2a:SetCost(Cost.DetachFromSelf(1,function(e,tp) return Duel.GetMatchingGroupCount(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,nil) end))
    effect::set_cost(
        d,
        e2a,
        Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::F(std::sync::Arc::new(detach_max))), None)),
    );
    // e2a:SetTarget(s.tdtg)
    effect::set_target(d, e2a, Some(cost_target(tdtg)));
    // e2a:SetOperation(s.tdop)
    effect::set_operation(d, e2a, Some(operation(tdop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_HAND) end)
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Each time your opponent makes a card(s) leave their Deck, except by drawing them: You can attach the top card of their Deck to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.attachcon)
    effect::set_condition(d, e3, Some(condition(attachcon)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `Card.IsAbleToDeck` as a filter
fn able_to_deck(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_deck(d, c)
}
/// The maximum of the detach cost: `Duel.GetMatchingGroupCount(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,nil)`
fn detach_max(d: &mut Duel, _e: EffectId, tp: u8) -> i64 {
    duel::get_matching_group_count(d, Some(&able_to_deck), tp, 0, LOCATION_ONFIELD, NO_EX) as i64
}
/// `#e:GetChainData().cost_detached_materials`
fn detached_count(d: &mut Duel, e: EffectId) -> usize {
    chain::effect_data(d, e, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned().nil().len()
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsXyzSummoned() end`
fn e2a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_HAND) end`
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_location(d, c, LOCATION_HAND), 1, NO_EX)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&able_to_deck), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local detached_materials_count=#e:GetChainData().cost_detached_materials
    let n = detached_count(d, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,detached_materials_count,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, n as u8, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local detached_materials_count=#e:GetChainData().cost_detached_materials
    let n = detached_count(d, a.e);
    // if not Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,detached_materials_count,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&able_to_deck), a.tp, 0, LOCATION_ONFIELD, n as u32, NO_EX) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,detached_materials_count,detached_materials_count,nil)
    let mut g = duel::select_matching_card(d, a.tp, Some(&able_to_deck), a.tp, 0, LOCATION_ONFIELD, n as u16, n as u16, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and #g:Match(Card.IsLocation,nil,LOCATION_DECK)>0 then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
        group::match_(d, &mut g, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        if !g.is_empty() {
            // local your_g,opp_g=g:Split(Card.IsControler,nil,tp)
            let (your_g, opp_g) = group::split(d, &g, &|d, c| card::is_controler(d, c, a.tp), NO_EX);
            // if #your_g>1 then Duel.SortDeckbottom(tp,tp,#your_g) end
            if your_g.len() > 1 {
                duel::sort_deckbottom(d, a.tp, a.tp, your_g.len() as u16);
            }
            // if #opp_g>1 then Duel.SortDeckbottom(tp,1-tp,#opp_g) end
            if opp_g.len() > 1 {
                duel::sort_deckbottom(d, a.tp, 1 - a.tp, opp_g.len() as u16);
            }
        }
    }
}
/// `s.attachconfilter(c,opp)`
fn attachconfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsPreviousControler(opp) and c:IsPreviousLocation(LOCATION_DECK) and c:IsReasonPlayer(opp) and not c:IsReason(REASON_DRAW)
    card::is_previous_controler(d, c, opp)
        && card::is_previous_location(d, c, LOCATION_DECK)
        && card::is_reason_player(d, c, opp)
        && !card::is_reason(d, c, REASON_DRAW)
}
/// `s.attachcon(e,tp,eg,ep,ev,re,r,rp)`
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    let opp = 1 - a.tp;
    // return eg:IsExists(s.attachconfilter,1,nil,1-tp) and (not eg:IsContains(c) or c:IsPreviousLocation(LOCATION_MZONE))
    group::is_exists(d, &eg, &|d, x| attachconfilter(d, x, opp), 1, NO_EX)
        && (!group::is_contains(&eg, c) || card::is_previous_location(d, c, LOCATION_MZONE))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsXyzMonster() and Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_xyz_monster(d, h) && duel::get_field_group_count(d, 1 - a.tp, LOCATION_DECK, 0) > 0;
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    // if c:IsRelateToEffect(e) and #g>0 then
    if card::is_relate_to_effect(d, c, a.e) && !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
