//! Geonator Transverser
use super::prelude::*;

pub const ID: u32 = 52119435;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinked() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_linked(d, h)
        })),
    );
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():GetLinkedGroupCount()==2 end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_linked_group_count(d, h) == 2
        })),
    );
    // e2:SetTarget(s.controltg)
    effect::set_target(d, e2, Some(cost_target(controltg)));
    // e2:SetOperation(s.controlop)
    effect::set_operation(d, e2, Some(operation(controlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local oc=e:GetHandler()
    let oc = effect::get_handler(d, e).nil();
    // return c==oc or oc:GetLinkedGroup():IsContains(c)
    if c == oc {
        return true;
    }
    let lg = card::get_linked_group(d, oc);
    group::is_contains(&lg, c)
}
// function s.controltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn controltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, h);
    // if chk==0 then return g:FilterCount(Card.IsAbleToChangeControler,nil)==2 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &|d, c| card::is_able_to_change_controler(d, c), NO_EX) == 2;
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 2, tp, 0);
    true
}
// function s.controlop(e,tp,eg,ep,ev,re,r,rp)
fn controlop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 or g:GetClassCount(Card.GetControler)==1 then return end
    if g.len() != 2 || group::get_class_count(d, &g, &|d, c| card::get_controler(d, c) as i64) == 1 {
        return;
    }
    // Duel.SwapControl(g:GetFirst(),g:GetNext())
    let v = group::iter(d, &g);
    duel::swap_control(d, v[0], v[1], 0, 0);
}
