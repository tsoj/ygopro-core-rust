//! Verdict of Anubis
use super::prelude::*;

pub const ID: u32 = 59576447;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TEMPLE_OF_THE_KINGS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of an opponent's Spell/Trap Card, and if you do, destroy it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,3,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    a.ep == 1 - a.tp
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, 0, 3, h.into())
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TEMPLE_OF_THE_KINGS),tp,LOCATION_ONFIELD,0,1,nil) then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil())
            && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS])),
                a.tp,
                LOCATION_ONFIELD,
                0,
                1,
                NO_EX,
            )
        {
            // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,0,LOCATION_MZONE,nil)
            let g = duel::get_matching_group(d, Some(&card::is_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
            // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
            if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Destroy(g,REASON_EFFECT)>0 then
            if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
                // local dam=Duel.GetOperatedGroup():GetSum(Card.GetBaseAttack)/2
                let og = duel::get_operated_group(d);
                let sum = group::get_sum(d, &og, &|d, c| card::get_base_attack(d, c) as i64);
                let dam = sum as f64 / 2.0;
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Damage(1-tp,dam,REASON_EFFECT)
                duel::damage(d, 1 - a.tp, lua_int(dam), REASON_EFFECT, false, None);
            }
        }
    }
}
