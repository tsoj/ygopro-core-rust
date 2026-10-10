//! No.49 Fortune Tune
use super::prelude::*;

pub const ID: u32 = 16259549;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(49), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 3 monsters
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // Neither player can target this card on the field with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card would be destroyed by battle or card effect, you can detach 1 material from this card instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, during your Standby Phase: Gain 500 LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e3:SetTarget(s.rectg)
    effect::set_target(d, e3, Some(cost_target(rectg)));
    // e3:SetOperation(s.recop)
    effect::set_operation(d, e3, Some(operation(recop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // If this card is sent from the field to the GY: Target 2 Level 3 monsters in your GY; shuffle them both into the Deck, and if you do, return this card from your GY to the Extra Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(e,tp) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_ONFIELD)
    })));
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    true
}
// function s.recop(e,tp,eg,ep,ev,re,r,rp)
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:GetLevel()==3 and c:IsCanBeEffectTarget(e) and c:IsAbleToDeck()
    card::get_level(d, c) == 3 && card::is_can_be_effect_target(d, c, Some(e)) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,2,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(sg), 2, a.tp, 0);
    }
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    // if not g then return end
    let Some(g) = duel::get_target_cards(d, a.e) else {
        return;
    };
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)==2 then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX) == 2
    {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
