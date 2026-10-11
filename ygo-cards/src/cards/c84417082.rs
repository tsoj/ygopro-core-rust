//! Ｎｏ．９１ サンダー・スパーク・ドラゴン
//! Number 91: Thunder Spark Dragon
use super::prelude::*;

pub const ID: u32 = 84417082;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(91), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.Choice(
    //     {Cost.DetachFromSelf(3),aux.Stringid(id,1),s.desmonscheck},
    //     {Cost.DetachFromSelf(5),aux.Stringid(id,2),s.desallcheck}
    // ))
    effect::set_cost(
        d,
        e1,
        Some(cost::choice(vec![
            cost::CostChoice {
                cost: cost::detach_from_self(cost::Count::N(3), None, None),
                desc: aux::stringid(ID, 1),
                check: Some(cost_target(desmonscheck)),
            },
            cost::CostChoice {
                cost: cost::detach_from_self(cost::Count::N(5), None, None),
                desc: aux::stringid(ID, 2),
                check: Some(cost_target(desallcheck)),
            },
        ])),
    );
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desmonscheck(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desmonscheck(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into())
}
/// `s.desallcheck(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desallcheck(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0
    duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0
}
/// `e:GetChainData().cost_choice==1`
fn cost_choice_is_1(d: &mut Duel, e: EffectId) -> bool {
    chain::effect_data(d, e, None, chain::field::COST_CHOICE).int() == 1
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=e:GetChainData().cost_choice==1
    //   and Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    //   or Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    let g = if cost_choice_is_1(d, a.e) {
        duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, h.into())
    } else {
        duel::get_field_group(d, tp, 0, LOCATION_ONFIELD)
    };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc: Option<CardId> = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // local g=e:GetChainData().cost_choice==1
    //   and Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,exc)
    //   or Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = if cost_choice_is_1(d, a.e) {
        let ex = match exc {
            Some(x) => x.into(),
            None => NO_EX,
        };
        duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, ex)
    } else {
        duel::get_field_group(d, tp, 0, LOCATION_ONFIELD)
    };
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
