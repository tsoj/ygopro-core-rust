//! Sylvan Waterslide
use super::prelude::*;

pub const ID: u32 = 49838105;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate the top card of your Deck when the opponent attacks directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Replace your normal draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DECKDES+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_DECKDES + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e3, EVENT_PREDRAW);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.cfcon)
    effect::set_condition(d, e3, Some(condition(cfcon)));
    // e3:SetTarget(s.cftg)
    effect::set_target(d, e3, Some(cost_target(cftg)));
    // e3:SetOperation(s.cfop)
    effect::set_operation(d, e3, Some(operation(cfop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
// function s.target2(e,tp,eg,ep,ev,re,r,rp,chk)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 or not e:GetHandler():IsRelateToEffect(e) then return end
    if effect::get_label(d, a.e) == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // if tc:IsRace(RACE_PLANT) then
    if card::is_race(d, tc, RACE_PLANT) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(tc,REASON_EFFECT+REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT + REASON_EXCAVATE, None, None);
    } else {
        // Duel.MoveSequence(tc,1)
        duel::move_sequence(d, tc, 1, None);
    }
}
// function s.cfcon(e,tp,eg,ep,ev,re,r,rp)
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetDrawCount(tp)>0
    a.tp == duel::get_turn_player(d) && duel::get_draw_count(d, a.tp) > 0
}
// function s.cftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dt=Duel.GetDrawCount(tp)
    let dt = duel::get_draw_count(d, a.tp);
    // if dt~=0 then
    if dt != 0 {
        // _replace_count=0
        set_lua_global(d, "_replace_count", Value::Int(0));
        // _replace_max=dt
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_DRAW_COUNT)
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetReset(RESET_PHASE|PHASE_DRAW)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.cfop(e,tp,eg,ep,ev,re,r,rp)
fn cfop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // _replace_count=_replace_count+1
    let count = lua_global(d, "_replace_count").int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(count));
    // if _replace_count<=_replace_max then
    if count <= lua_global(d, "_replace_max").int() {
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, a.tp, 1);
        // local g=Duel.GetDecktopGroup(tp,1)
        let g = duel::get_decktop_group(d, a.tp, 1);
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if tc:IsRace(RACE_PLANT) then
        if card::is_race(d, tc, RACE_PLANT) {
            // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
            duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
        } else {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
        }
    }
}
