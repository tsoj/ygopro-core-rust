//! Angelechy Shatranga
//! JP name
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 42410161;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // You can target 1 monster your opponent controls; banish it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is placed in the Spell & Trap Zone as a Continuous Spell: You can add 1 "Angelechy" Trap from your Deck or GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_STZONE)
    effect::set_range(d, e2, LOCATION_STZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While this card is treated as a Continuous Spell, your opponent can only attempt to activate up to 5 monster effects per turn
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3a, EVENT_CHAINING);
    // e3a:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3a, LOCATION_STZONE);
    // e3a:SetCondition(function(e) return e:GetHandler():IsContinuousSpell() end)
    effect::set_condition(d, e3a, Some(condition(e3a_condition)));
    // e3a:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3a, Some(operation(e3a_operation)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD);
    // e3b:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3b, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3b:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3b, EFFECT_CANNOT_ACTIVATE);
    // e3b:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3b, LOCATION_STZONE);
    // e3b:SetTargetRange(0,1)
    effect::set_target_range(d, e3b, 0, 1);
    // e3b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id,5) end)
    effect::set_condition(d, e3b, Some(condition(e3b_condition)));
    // e3b:SetValue(function(e,re) return re:IsMonsterEffect() end)
    effect::set_value_fn(d, e3b, lua_fn(e3b_value));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return eg:IsContains(c) and not c:IsPreviousLocation(LOCATION_SZONE)
    let eg = a.eg.clone().nil();
    eg.contains(c) && !card::is_previous_location(d, c, LOCATION_SZONE)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsContinuousSpell()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_continuous_spell(d, c) && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // if sc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(sc) end
        if card::is_location(d, sc, LOCATION_GRAVE) {
            duel::hint_selection(d, sc, true);
        }
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // if sc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,sc) end
        if card::is_previous_location(d, sc, LOCATION_DECK) {
            duel::confirm_cards(d, 1 - tp, sc);
        }
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) if ep==tp or not re:IsMonsterEffect() then return end
//     e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1) end
fn e3a_operation(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsMonsterEffect() then return end
    if a.ep == a.tp {
        return;
    }
    let Some(re) = a.re else { return };
    if !effect::is_monster_effect(d, re) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
// function(e) return e:GetHandler():IsContinuousSpell() end
fn e3a_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_continuous_spell(d, c)
}
// function(e) return e:GetHandler():HasFlagEffect(id,5) end
fn e3b_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, c, ID, Some(5))
}
// function(e,re) return re:IsMonsterEffect() end
fn e3b_value(d: &mut Duel, a: Args) -> bool {
    effect::is_monster_effect(d, a.e(1))
}
