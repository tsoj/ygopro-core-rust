//! Fiendsmith's Lacrima
use super::prelude::*;

pub const ID: u32 = 46640168;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 2)]);
    // Monsters your opponent controls lose 600 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(-600)
    effect::set_value(d, e1, -600);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand or Special Summon 1 LIGHT Fiend from your GY or banishment
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e2, Some(condition(fusion_summoned_cond)));
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 1200 damage to your opponent
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.damcost)
    effect::set_cost(d, e3, Some(cost_target(damcost)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,fc,sumtype,tp) and c:IsRace(RACE_FIEND,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_FIEND, Some(fc), sumtype, Some(tp))
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.thspfilter(c,e,tp,ft)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FIEND) and c:IsFaceup()
    //     and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_FIEND)
        && card::is_faceup(d, c)
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,...,chk,chkc)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.thspfilter(chkc,e,tp,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thspfilter(d, chkc, a.e, a.tp, ft);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thspfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local g=Duel.SelectTarget(tp,s.thspfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp,ft)
    let g = duel::select_target(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g.clone(), 1, a.tp, 0);
    // if g:GetFirst():IsLocation(LOCATION_GRAVE) then
    let first = group::get_first(d, &g.clone().nil()).nil();
    if card::is_location(d, first, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, a.tp, 0);
    }
    true
}
/// `s.thspop(e,tp,...)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // aux.ToHandOrElse(tc,tp,
    //     function()
    //         return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     end,
    //     function()
    //         Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
    //     end,
    //     aux.Stringid(id,3) --"Special Summon it"
    // )
    aux::to_hand_or_else(
        d,
        tc,
        a.tp,
        Some(&|d, _c| {
            duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None)
        }),
        Some(&|d, _g| duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
/// `s.damcostfilter(c,tp)`
fn damcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FIEND) and c:IsAbleToDeckOrExtraAsCost()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FIEND) && card::is_able_to_deck_or_extra_as_cost(d, c)
}
/// `s.damcost(e,tp,...,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.damcostfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&damcostfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.damcostfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_matching_card(d, a.tp, Some(&damcostfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
/// `s.damtg(e,tp,...,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1200)
    duel::set_target_param(d, 1200);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1200);
    true
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
