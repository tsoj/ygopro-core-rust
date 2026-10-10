//! Void Trap Hole
use super::prelude::*;

pub const ID: u32 = 1005587;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.disfilter(c,tp)`
fn disfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsAttackAbove(2000) and c:IsNegatableMonster()
    card::is_summon_player(d, c, 1 - tp) && card::is_attack_above(d, c, 2000) && card::is_negatable_monster(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.disfilter,nil,tp)
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| disfilter(d, c, a.tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // for sc in g:Iter() do
    for sc in group::iter(d, &g) {
        // sc:CreateEffectRelation(e)
        card::create_effect_relation(d, sc, a.e);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=eg:Filter(s.disfilter,nil,tp):Match(Card.IsRelateToEffect,nil,e)
    let mut g = group::filter(d, &a.eg.clone().nil(), &|d, c| disfilter(d, c, tp), NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sc=nil
    // if #g==1 then
    let sc = if g.len() == 1 {
        // sc=g:GetFirst()
        group::get_first(d, &g).nil()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        sc
    };
    // if sc:IsCanBeDisabledByEffect(e) then
    if card::is_can_be_disabled_by_effect(d, sc, a.e, true) {
        // sc:NegateEffects(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, sc, handler, None, false, None);
        // Duel.AdjustInstantly(sc)
        duel::adjust_instantly(d, Some(sc));
        // if sc:IsDisabled() then
        if card::is_disabled(d, sc) {
            // Duel.Destroy(sc,REASON_EFFECT)
            duel::destroy(d, sc, REASON_EFFECT, None, None);
        }
    }
}
