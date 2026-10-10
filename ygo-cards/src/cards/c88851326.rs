//! Legacy of the Duelist
use super::prelude::*;

pub const ID: u32 = 88851326;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent set
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e3:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e3, EFFECT_CANNOT_SSET);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.setcon1)
    effect::set_condition(d, e3, Some(condition(setcon1)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(effect_card_fn(settg)));
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetCondition(s.setcon2)
    effect::set_condition(d, e4, Some(condition(setcon2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Prevent attack
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e5, EFFECT_CANNOT_ATTACK);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e5, LOCATION_MZONE, LOCATION_MZONE);
    // e5:SetTarget(s.attg)
    effect::set_target(d, e5, Some(effect_card_fn(attg)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Add tohand
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e6, CATEGORY_TOHAND);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e6, EVENT_PREDRAW);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.thcon)
    effect::set_condition(d, e6, Some(condition(thcon)));
    // e6:SetTarget(s.thtg)
    effect::set_target(d, e6, Some(cost_target(thtg)));
    // e6:SetOperation(s.thop)
    effect::set_operation(d, e6, Some(operation(thop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Check for single Set
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SSET)
        effect::set_code(d, ge1, EVENT_SSET);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(Card.IsPreviousLocation,1,nil,LOCATION_HAND) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_HAND), 1, NO_EX) {
        // local g=eg:Filter(Card.IsPreviousLocation,nil,LOCATION_HAND)
        let g = group::filter(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_HAND), NO_EX);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // if tc:GetFlagEffect(id)==0 then
            if card::get_flag_effect(d, tc, ID) == 0 {
                // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET),0,1)
                card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 0, 1, 0, 0);
                // Duel.RegisterFlagEffect(rp,id,RESET_PHASE|PHASE_END,0,1)
                duel::register_flag_effect(d, a.rp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
        }
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(tp)
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, a.tp)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(1-tp) and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, 1 - a.tp) && card::is_spell_trap(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.NegateAttack() and tc:IsRelateToEffect(e) then
    if duel::negate_attack(d) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.setcon1(e)`
fn setcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)>0
    let hp = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, hp, ID) > 0
}
/// `s.setcon2(e)`
fn setcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(1-e:GetHandlerPlayer(),id)>0
    let hp = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, 1 - hp, ID) > 0
}
/// `s.settg(e,c)`
fn settg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND)
    card::is_location(d, c, LOCATION_HAND)
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //     and Duel.GetDrawCount(tp)>0
    duel::get_turn_player(d) == a.tp
        && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0
        && duel::get_draw_count(d, a.tp) > 0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local dt=Duel.GetDrawCount(tp)
    let dt = duel::get_draw_count(d, a.tp);
    // if dt==0 or not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if dt == 0 || !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // _replace_count=1
    set_lua_global(d, "_replace_count", Value::Int(1));
    // _replace_max=dt
    set_lua_global(d, "_replace_max", Value::Int(dt as i64));
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
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
    // if _replace_count>_replace_max then return end
    if lua_global(d, "_replace_count").int() > lua_global(d, "_replace_max").int() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
