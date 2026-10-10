//! Freed the Matchless General
use super::prelude::*;

pub const ID: u32 = 49681811;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // self destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // search
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e4, EVENT_PREDRAW);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.condition)
    effect::set_condition(d, e4, Some(condition(cond)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if not c:IsSpell() or c:GetCardTargetCount()==0 then return false end
    if !card::is_spell(d, c) || card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // return c:GetCardTarget():IsContains(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    card::get_card_target(d, c).contains(h)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not re:IsSpellEffect() then return end
    let re = a.re.nil();
    if !effect::is_spell_effect(d, re) {
        return;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info.first().and_then(|v| v.as_group().cloned());
    // if not g or #g==0 then return end
    let g = match g {
        Some(g) if !g.is_empty() => g,
        _ => return,
    };
    // if g:IsContains(e:GetHandler()) then
    let h = effect::get_handler(d, a.e).nil();
    if g.contains(h) {
        // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_effect(d, a.ev as u8) {
            let rh = effect::get_handler(d, re).nil();
            if card::is_relate_to_effect(d, rh, re) {
                // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
                duel::destroy(d, rh, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //   and Duel.GetDrawCount(tp)>0
    a.tp == duel::get_turn_player(d) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, a.tp) > 0
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_WARRIOR) and c:IsAbleToHand()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_WARRIOR as u64) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
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
        let h = effect::get_handler(d, a.e).nil();
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
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // _replace_count=_replace_count+1
    let count = lua_global(d, "_replace_count").int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(count));
    // if _replace_count>_replace_max or not e:GetHandler():IsRelateToEffect(e) or e:GetHandler():IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    let max = lua_global(d, "_replace_max").int();
    if count > max || !card::is_relate_to_effect(d, h, a.e) || card::is_facedown(d, h) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
