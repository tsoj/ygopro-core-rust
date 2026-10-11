//! Spellbook Library of the Heliosphere
use super::prelude::*;

pub const ID: u32 = 20822520;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_CHAINING)
        effect::set_code(d, ge1, EVENT_CHAINING);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and not re:GetHandler():IsSetCard(SET_SPELLBOOK) then
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_effect(d, re)
        && !card::is_set_card(d, h, &[SET_SPELLBOOK as u16])
    {
        // Duel.RegisterFlagEffect(rp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.rp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSetCard(SET_SPELLBOOK)
    card::is_spell(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16])
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,5,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 5, NO_EX)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFlagEffect(tp,id+1)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID + 1) == 0;
    }
    // oath effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and not re:GetHandler():IsSetCard(SET_SPELLBOOK)
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    if !effect::is_spell_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    !card::is_set_card(d, h, &[SET_SPELLBOOK as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<2 then return false end
        if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < 2 {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,2)
        let g = duel::get_decktop_group(d, tp, 2);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSetCard(SET_SPELLBOOK)
    card::is_spell(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.ConfirmDecktop(p,2)
    duel::confirm_decktop(d, p, 2);
    // local g=Duel.GetDecktopGroup(p,2)
    let g = duel::get_decktop_group(d, p, 2);
    // if #g>0 then
    if !g.is_empty() {
        // local sg=g:Filter(s.filter,nil)
        let sg = group::filter(d, &g, &filter, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // if sg:GetFirst():IsAbleToHand() then
            let first = group::get_first(d, &sg).nil();
            if card::is_able_to_hand(d, first, None) {
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-p,sg)
                duel::confirm_cards(d, 1 - p, sg.clone());
                // Duel.ShuffleHand(p)
                duel::shuffle_hand(d, p);
            } else {
                // Duel.SendtoGrave(sg,REASON_EFFECT)
                duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
            }
        }
        // Duel.ShuffleDeck(p)
        duel::shuffle_deck(d, p);
    }
}
