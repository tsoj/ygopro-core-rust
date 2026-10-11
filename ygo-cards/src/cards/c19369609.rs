//! Lyrilusc - Promenade Thrush
//! scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 19369609;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 1 monsters
    // Xyz.AddProcedure(c,nil,1,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Gains 500 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(function(e) return e:GetHandler():GetOverlayCount()*500 end)
    effect::set_value_fn(d, e1, lua_fn(atk_val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 Spell/Trap your opponent controls into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make 1 monster you control gain 300 ATK for each material detached
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(Cost.DetachFromSelf(1,function(e) return e:GetHandler():GetOverlayCount() end,function(e,og) e:SetLabel(#og) end))
    effect::set_cost(
        d,
        e3,
        Some(cost::detach_from_self(
            cost::Count::N(1),
            Some(cost::Count::F(std::sync::Arc::new(overlay_count_fn))),
            Some(std::sync::Arc::new(set_label_count_fn)),
        )),
    );
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount()*500 end`
fn atk_val(d: &mut Duel, a: Args) -> i64 {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) as i64 * 500
}
/// `function(e) return e:GetHandler():GetOverlayCount() end` (the max of Cost.DetachFromSelf)
fn overlay_count_fn(d: &mut Duel, e: EffectId, _tp: u8) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_overlay_count(d, h) as i64
}
/// `function(e,og) e:SetLabel(#og) end` (the operation of Cost.DetachFromSelf)
fn set_label_count_fn(d: &mut Duel, e: EffectId, og: &CardSet) {
    effect::set_label(d, e, &[og.len() as i64]);
}
/// `aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsSpellTrap() and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    // return bc and bc:IsFaceup() and bc~=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    match duel::get_battle_monster(d, a.tp).0 {
        Some(bc) => card::is_faceup(d, bc) && bc != h,
        None => false,
    }
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetBattleMonster(tp)
    let bc = duel::get_battle_monster(d, a.tp).0;
    // if bc and bc:IsFaceup() and bc:IsControler(tp) and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_faceup(d, bc) && card::is_controler(d, bc, a.tp) && card::is_relate_to_battle(d, bc) {
            let h = effect::get_handler(d, a.e).nil();
            // bc:UpdateAttack(e:GetLabel()*300,RESETS_STANDARD_PHASE_END,e:GetHandler())
            let amt = effect::get_label(d, a.e) * 300;
            card::update_attack(d, bc, amt, Some(RESETS_STANDARD_PHASE_END), Some(h), None);
        }
    }
}
