//! Duel Evolution - Assault Zone
use super::prelude::*;

pub const ID: u32 = 91002901;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ASSAULT_MODE], listed_series: &[SET_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can add 1 monster that mentions "Assault Mode Activate" from your Deck or GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "/Assault Mode" monsters cannot be destroyed by battle or your opponent's card effects during the turn they are Special Summoned
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2a, LOCATION_FZONE);
    // e2a:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2a, LOCATION_MZONE, LOCATION_MZONE);
    // e2a:SetTarget(function(e,c) return c:IsSetCard(SET_ASSAULT_MODE) and c:IsStatus(STATUS_SPSUMMON_TURN) end)
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16]) && card::is_status(d, c, STATUS_SPSUMMON_TURN))));
    // e2a:SetValue(1)
    effect::set_value(d, e2a, 1);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2b:SetValue(aux.indoval)
    effect::set_value_fn(d, e2b, aux::indoval());
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Apply an "once this turn, you can activate "Assault Mode Activate" by Tributing a Synchro Monster in the Extra Deck" effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:ListsCode(CARD_ASSAULT_MODE) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_ASSAULT_MODE]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // Once this turn, you can activate "Assault Mode Activate" by Tributing a Synchro Monster in the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_EXTRA_RELEASE_NONSUM)
    effect::set_code(d, e1, EFFECT_EXTRA_RELEASE_NONSUM);
    // e1:SetTargetRange(LOCATION_EXTRA,0)
    effect::set_target_range(d, e1, LOCATION_EXTRA, 0);
    // e1:SetCondition(function(e) return not Duel.HasFlagEffect(e:GetHandlerPlayer(),id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        !duel::has_flag_effect(d, p, ID, None)
    })));
    // e1:SetValue(s.relval)
    effect::set_value_fn(d, e1, lua_fn(relval_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Manually handling the OPT of the above effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return s.relval(e,re,r) and eg:IsExists(Card.IsPreviousLocation,1,nil,LOCATION_EXTRA) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        relval(d, a.re, a.r) && {
            let eg = a.eg.clone().nil();
            group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_EXTRA), 1, NO_EX)
        }
    })));
    // e2:SetOperation(function(e,tp) Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    })));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
// function s.relval(e,re,r,rp)
fn relval_value(d: &mut Duel, a: Args) -> bool {
    relval(d, a.v(1).as_effect(), a.int(2) as u32)
}
// return re and re:IsActivated() and (r&REASON_COST)>0 and re:GetHandler():IsCode(CARD_ASSAULT_MODE)
fn relval(d: &mut Duel, re: Option<EffectId>, r: u32) -> bool {
    match re {
        None => false,
        Some(re) => {
            effect::is_activated(d, re) && (r & REASON_COST) > 0 && {
                let h = effect::get_handler(d, re).nil();
                card::is_code(d, h, &[CARD_ASSAULT_MODE])
            }
        }
    }
}
