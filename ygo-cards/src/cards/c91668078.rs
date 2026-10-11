//! In Papa's Footsteps
use super::prelude::*;

pub const ID: u32 = 91668078;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish up to 5 cards from your GY face-down
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.NOT(Card.IsCode),1,nil,id)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| !card::is_code(d, c, &[ID]), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_GRAVE,0,1,nil,tp,POS_FACEDOWN,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
            tp,
            LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local your_rg=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_GRAVE,0,1,5,nil,tp,POS_FACEDOWN,REASON_EFFECT)
    let your_rg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
        tp,
        LOCATION_GRAVE,
        0,
        1,
        5,
        false,
        NO_EX,
    )
    .nil();
    // if #your_rg==0 then return end
    if your_rg.is_empty() {
        return;
    }
    // Duel.HintSelection(your_rg,true)
    duel::hint_selection(d, &your_rg, true);
    // if Duel.Remove(your_rg,POS_FACEDOWN,REASON_EFFECT)>0 and Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_REMOVED,0,7,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil,1-tp,POS_FACEDOWN,REASON_EFFECT) then
    if duel::remove(d, &your_rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_REMOVED, 0, 7, NO_EX)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(1 - tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
            tp,
            0,
            LOCATION_GRAVE,
            1,
            NO_EX,
        )
    {
        // local oppo_rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,nil,1-tp,POS_FACEDOWN,REASON_EFFECT)
        let mut oppo_rg = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(1 - tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
            tp,
            0,
            LOCATION_GRAVE,
            NO_EX,
        );
        // if #oppo_rg>5 then
        if oppo_rg.len() > 5 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_REMOVE as u64);
            // oppo_rg=oppo_rg:FilterSelect(1-tp,Card.IsAbleToRemove,5,5,nil,1-tp,POS_FACEDOWN,REASON_EFFECT)
            oppo_rg = group::filter_select(
                d,
                &oppo_rg,
                1 - tp,
                &|d, c| card::is_able_to_remove(d, c, Some(1 - tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT)),
                5,
                5,
                false,
                NO_EX,
            )
            .nil();
        }
        // Duel.HintSelection(oppo_rg,true)
        duel::hint_selection(d, &oppo_rg, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(oppo_rg,POS_FACEDOWN,REASON_EFFECT,nil,1-tp)
        duel::remove(d, &oppo_rg, POS_FACEDOWN as u8, REASON_EFFECT, None, Some(1 - tp));
    }
}
