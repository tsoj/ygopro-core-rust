//! Superheavy Samurai General Coral
use super::prelude::*;

pub const ID: u32 = 78274190;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // scale
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_LSCALE)
    effect::set_code(d, e2, EFFECT_CHANGE_LSCALE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCondition(s.sccon)
    effect::set_condition(d, e2, Some(condition(sccon)));
    // e2:SetValue(4)
    effect::set_value(d, e2, 4);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CHANGE_RSCALE)
    effect::set_code(d, e3, EFFECT_CHANGE_RSCALE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // chain attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetRange(LOCATION_PZONE)
    effect::set_range(d, e4, LOCATION_PZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.catg)
    effect::set_target(d, e4, Some(cost_target(catg)));
    // e4:SetOperation(s.caop)
    effect::set_operation(d, e4, Some(operation(caop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // draw
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e5, CATEGORY_DRAW);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.condition)
    effect::set_condition(d, e5, Some(condition(cond)));
    // e5:SetCost(s.cost)
    effect::set_cost(d, e5, Some(cost_target(cost_fn)));
    // e5:SetTarget(s.target)
    effect::set_target(d, e5, Some(cost_target(target)));
    // e5:SetOperation(s.operation)
    effect::set_operation(d, e5, Some(operation(op)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.sccon(e)
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.afilter(c,tp)
fn afilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsSetCard(SET_SUPERHEAVY_SAMURAI) and c:CanChainAttack()
    card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16]) && card::can_chain_attack(d, c, None, false)
}
// function s.catg(e,tp,eg,ep,ev,re,r,rp,chk)
fn catg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.afilter,1,nil,tp) end
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| afilter(d, c, tp), 1, NO_EX);
    }
    // local a=eg:Filter(s.afilter,nil,tp):GetFirst()
    let filtered = group::filter(d, &eg, &|d, c| afilter(d, c, tp), NO_EX);
    let a_card = group::get_first(d, &filtered);
    // Duel.SetTargetCard(a)
    if let Some(a_card) = a_card {
        duel::set_target_card(d, a_card);
    }
    true
}
// function s.caop(e,tp,eg,ep,ev,re,r,rp)
fn caop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsControler(tp) and tc:IsRelateToBattle() then
    if card::is_faceup(d, tc) && card::is_controler(d, tc, tp) && card::is_relate_to_battle(d, tc) {
        // Duel.ChainAttack()
        duel::chain_attack(d, None);
    }
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,nil,SET_SUPERHEAVY_SAMURAI) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])), 1, 1, false, None, NO_EX);
    }
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let mut ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if ct>2 then ct=2 end
    if ct > 2 {
        ct = 2;
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,ct,false,nil,nil,SET_SUPERHEAVY_SAMURAI)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])), 1, ct as usize, false, None, NO_EX);
    // local rct=Duel.Release(g,REASON_COST)
    let rct = duel::release(d, g, REASON_COST, None);
    // e:SetLabel(rct)
    effect::set_label(d, a.e, &[rct as i64]);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, label as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,e:GetLabel(),REASON_EFFECT)
    let label = effect::get_label(d, a.e);
    duel::draw(d, a.tp, label as u32, REASON_EFFECT);
}
