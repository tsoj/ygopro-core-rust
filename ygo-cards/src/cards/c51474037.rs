//! Yamorimori
//! scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 51474037;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c,e,tp)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsFaceup()
    //     and ((c:IsControler(tp) and c:IsRace(RACE_REPTILE))
    //     or (c:IsControler(1-tp) and (c:IsCanTurnSet() or c:GetAttack()>0)))
    card::is_can_be_effect_target(d, c, Some(e))
        && card::is_faceup(d, c)
        && ((card::is_controler(d, c, tp) && card::is_race(d, c, RACE_REPTILE))
            || (card::is_controler(d, c, 1 - tp) && (card::is_can_turn_set(d, c) || card::get_attack(d, c) > 0)))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // if sg:GetClassCount(Card.GetControler)~=2 then return false end
    if group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) != 2 {
        return (false, false);
    }
    // local opp_c=sg:Filter(Card.IsControler,nil,1-tp):GetFirst()
    let opp = group::filter(d, sg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    let opp_c = group::get_first(d, &opp).nil();
    // return sg:FilterCount(Card.IsCanTurnSet,nil)==2 or opp_c:GetAttack()>0
    let res = group::filter_count(d, sg, &|d, c| card::is_can_turn_set(d, c), NO_EX) == 2 || card::get_attack(d, opp_c) > 0;
    (res, false)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_FACEUP)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_FACEUP as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local self_g,opp_g=tg:Split(Card.IsControler,nil,tp)
    let (self_g, opp_g) = group::split(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // local b1=tg:IsExists(Card.IsCanTurnSet,2,nil)
    let b1 = group::is_exists(d, &tg, &|d, c| card::is_can_turn_set(d, c), 2, NO_EX);
    // local b2=opp_g:GetFirst():GetAttack()>0
    let opp_first = group::get_first(d, &opp_g).nil();
    let b2 = card::get_attack(d, opp_first) > 0;
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    let op_num = op.map_or(0, |v| v as i64);
    // if op==1 then
    if op_num == 1 {
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, e, CATEGORY_POSITION + CATEGORY_SET);
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,tg,2,tp,POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(tg), 2, tp, POS_FACEDOWN_DEFENSE as i32);
    // elseif op==2 then
    } else if op_num == 2 {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
        effect::set_category(d, e, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
        // e:SetLabelObject(self_g:GetFirst())
        let self_first = match group::get_first(d, &self_g) {
            Some(c) => Value::Card(c),
            None => Value::Nil,
        };
        effect::set_label_object(d, e, self_first);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,self_g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(self_g), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,opp_g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(opp_g), 1, tp, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, e, &[op_num]);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    // if #g==0 then return end
    let g = duel::get_target_cards(d, e).nil();
    if g.is_empty() {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    // elseif op==2 then
    } else if op == 2 {
        // local self_c=e:GetLabelObject()
        let self_c = effect::get_label_object(d, e).as_card().nil();
        // local opp_c=g:RemoveCard(self_c):GetFirst()
        let mut rest = g.clone();
        group::remove_card(&mut rest, &CardSet::from_card(self_c));
        let opp_c = group::get_first(d, &rest);
        // if self_c:IsRelateToEffect(e) and self_c:IsControler(tp) and Duel.Destroy(self_c,REASON_EFFECT)>0
        if card::is_relate_to_effect(d, self_c, e)
            && card::is_controler(d, self_c, tp)
            && duel::destroy(d, self_c, REASON_EFFECT, None, None) > 0
        {
            // and opp_c and opp_c:IsRelateToEffect(e) and opp_c:IsFaceup() then
            if let Some(opp_c) = opp_c {
                if card::is_relate_to_effect(d, opp_c, e) && card::is_faceup(d, opp_c) {
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
                    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
                    // e1:SetValue(0)
                    effect::set_value(d, e1, 0);
                    // e1:SetReset(RESETS_STANDARD_PHASE_END)
                    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                    // opp_c:RegisterEffect(e1)
                    card::register_effect(d, opp_c, e1, false);
                }
            }
        }
    }
}
