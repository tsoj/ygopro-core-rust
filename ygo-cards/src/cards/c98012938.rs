//! Vulcan the Divine
use super::prelude::*;

pub const ID: u32 = 98012938;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // return
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SYNCHRO
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil) then
    let faceup_hand = |d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None);
    if duel::is_existing_target(d, Some(&faceup_hand), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_target(d, Some(&faceup_hand), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g1=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_ONFIELD,0,1,1,nil)
        let mut g1 = duel::select_target(d, tp, Some(&faceup_hand), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g2=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,1,nil)
        let g2 = duel::select_target(d, tp, Some(&faceup_hand), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g1,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), 2, 0, 0);
    }
    true
}
fn hfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRelateToEffect(e) and c:IsFaceup()
    card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // g=g:Filter(s.hfilter,nil,e)
    let g = group::filter(d, &g, &|d, x| hfilter(d, x, e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // if tc:IsLocation(LOCATION_HAND) then
            if card::is_location(d, tc, LOCATION_HAND) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
                effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
                // e1:SetTargetRange(1,0)
                effect::set_target_range(d, e1, 1, 0);
                // e1:SetValue(s.aclimit)
                effect::set_value_fn(d, e1, lua_fn(aclimit));
                // e1:SetLabel(tc:GetCode())
                let code = card::get_code(d, tc)[0];
                effect::set_label(d, e1, &[code as i64]);
                // e1:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
            }
        }
    }
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsCode(e:GetLabel())
    let h = effect::get_handler(d, a.e(1)).nil();
    card::is_code(d, h, &[effect::get_label(d, a.e(0)) as u32])
}
