//! Devouring Sarcoughagus
use super::prelude::*;

pub const ID: u32 = 30037118;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control when self is destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.ctcon)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take control when another Zombie is destroyed by battle
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.gctcon)
    effect::set_condition(d, e2, Some(condition(gctcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabelObject(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    effect::set_label_object(d, a.e, Value::Card(c));
    // local c=e:GetHandler()
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsStatus(STATUS_OPPO_BATTLE) and bc and bc:IsAbleToChangeControler()
    card::is_status(d, c, STATUS_OPPO_BATTLE) && bc.is_some_and(|bc| card::is_able_to_change_controler(d, bc))
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject():GetReasonCard()
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let tc = card::get_reason_card(d, lo).nil();
    // if chk==0 then return tc:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local dc=e:GetLabelObject()
    // if not dc then return end
    let Some(dc) = effect::get_label_object(d, a.e).as_card() else {
        return;
    };
    // local tc=dc:GetBattleTarget()
    let tc = card::get_battle_target(d, dc);
    // if tc and tc:IsControler(1-tp) and Duel.GetControl(tc,tp) then
    if let Some(tc) = tc {
        if card::is_controler(d, tc, 1 - a.tp) && duel::get_control(d, tc, a.tp, 0, 0, None, None) != 0 {
            // Treat as Zombie
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetValue(RACE_ZOMBIE)
            effect::set_value(d, e1, RACE_ZOMBIE as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // ATK/DEF becomes 0
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
            // e2:SetValue(0)
            effect::set_value(d, e2, 0);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // local e3=e2:Clone()
            let e3 = effect::clone(d, e2);
            // e3:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e3, EFFECT_SET_DEFENSE_FINAL);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
/// `s.ctconfilter(c,tp)`
fn ctconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:GetPreviousRaceOnField()&RACE_ZOMBIE~=0 and c:IsStatus(STATUS_OPPO_BATTLE)
    //     and c:GetBattleTarget():IsAbleToChangeControler()
    card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && (card::get_previous_race_on_field(d, c) & RACE_ZOMBIE) != 0
        && card::is_status(d, c, STATUS_OPPO_BATTLE)
        && {
            let bc = card::get_battle_target(d, c).nil();
            card::is_able_to_change_controler(d, bc)
        }
}
/// `s.gctcon(e,tp,eg,ep,ev,re,r,rp)`
fn gctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not eg or eg:IsContains(e:GetHandler()) then return false end
    let Some(eg) = a.eg.clone() else {
        return false;
    };
    let h = effect::get_handler(d, a.e).nil();
    if group::is_contains(&eg, h) {
        return false;
    }
    // local g=eg:Filter(s.ctconfilter,nil,tp)
    let tp = a.tp;
    let g = group::filter(d, &eg, &|d, c| ctconfilter(d, c, tp), NO_EX);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // return #g>0
    !g.is_empty()
}
