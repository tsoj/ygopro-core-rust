//! Chronomaly Technology
use super::prelude::*;

pub const ID: u32 = 90951921;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
// function s.counterfilter(c)
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsSetCard(SET_CHRONOMALY)
    card::is_set_card(d, a.c(0), &[SET_CHRONOMALY as u16])
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // oath effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    true
}
// function s.splimit(e,c,sump,sumtype,sumpos,targetp,se)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSetCard(SET_CHRONOMALY)
    !card::is_set_card(d, a.c(1), &[SET_CHRONOMALY as u16])
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_CHRONOMALY) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsPlayerCanDiscardDeck(tp,2)
        //     or not Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) then return false end
        if !duel::is_player_can_discard_deck(d, a.tp, 2)
            || !duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
        {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,2)
        let g = duel::get_decktop_group(d, a.tp, 2);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 and Duel.IsPlayerCanDiscardDeck(tp,2) then
    let Some(tc) = tc else { return };
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0
        && duel::is_player_can_discard_deck(d, a.tp, 2)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetDecktopGroup(tp,2)
        let mut g = duel::get_decktop_group(d, a.tp, 2);
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g.clone());
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
            let sg = group::filter_select(d, &g, a.tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
            // g:Sub(sg)
            group::remove_card(&mut g, &sg);
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.DisableShuffleCheck()
                duel::disable_shuffle_check(d, true);
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
            }
            // if #g>0 then
            if !g.is_empty() {
                // Duel.DisableShuffleCheck()
                duel::disable_shuffle_check(d, true);
                // Duel.SendtoGrave(g,REASON_EFFECT)
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
