//! Maiden in Love
use super::prelude::*;

pub const ID: u32 = 8445808;

const COUNTER_MAIDEN: u16 = 0x1090;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Monsters your opponent controls that can attack must attack this card
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_FIELD)
    // e1:SetCode(EFFECT_MUST_ATTACK)
    // e1:SetTargetRange(0,LOCATION_MZONE)
    // e1:SetRange(LOCATION_MZONE)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_range(d, e1, LOCATION_MZONE);
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    // e2:SetValue(function(e,c) return c==e:GetHandler() end)
    // c:RegisterEffect(e2)
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| c == effect::get_handler(d, e).nil()));
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by battle
    // local e3=Effect.CreateEffect(c)
    // e3:SetType(EFFECT_TYPE_SINGLE)
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    // e3:SetRange(LOCATION_MZONE)
    // e3:SetValue(1)
    // c:RegisterEffect(e3)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // Activate 1 of these effects at the end of the Damage Step, if this card battled an opponent's monster
    // local e4=Effect.CreateEffect(c)
    // e4:SetDescription(aux.Stringid(id,0))
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    // e4:SetCondition(s.effcon)
    // e4:SetTarget(s.efftg)
    // e4:SetOperation(s.effop)
    // c:RegisterEffect(e4)
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e4, Some(condition(effcon)));
    effect::set_target(d, e4, Some(cost_target(efftg)));
    effect::set_operation(d, e4, Some(operation(effop)));
    card::register_effect(d, c, e4, false);
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetBattleTarget() and c:IsStatus(STATUS_OPPO_BATTLE)
    card::get_battle_target(d, c).is_some() && card::is_status(d, c, STATUS_OPPO_BATTLE)
}
// function s.controlfilter(c)
fn controlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasCounter(COUNTER_MAIDEN) and c:IsControlerCanBeChanged()
    card::has_counter(d, c, COUNTER_MAIDEN) && card::is_controler_can_be_changed(d, c, false, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,nil,COUNTER_MAIDEN,1)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_MAIDEN, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.controlfilter,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&controlfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsLocation(LOCATION_MZONE) and c:IsRelateToBattle() and (b1 or b2) end
    if a.chk == Some(0) {
        return card::is_location(d, c, LOCATION_MZONE) && card::is_relate_to_battle(d, c) && (b1 || b2);
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_COUNTER)
        effect::set_category(d, a.e, CATEGORY_COUNTER);
        // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,COUNTER_MAIDEN)
        duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, COUNTER_MAIDEN as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        // local g=Duel.GetMatchingGroup(s.controlfilter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&controlfilter), tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, tp, 0);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Place 1 Maiden Counter on 1 face-up monster your opponent controls.
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
        // local sc=Duel.SelectMatchingCard(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,1,nil,COUNTER_MAIDEN,1):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_MAIDEN, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // sc:AddCounter(COUNTER_MAIDEN,1)
            card::add_counter(d, sc, COUNTER_MAIDEN, 1, false);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Take control of 1 opponent's monster with a Maiden Counter.
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.controlfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&controlfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.GetControl(sc,tp)
            duel::get_control(d, sc, tp, 0, 0, None, None);
        }
    }
}
