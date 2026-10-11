//! Vampire Red Baron
use super::prelude::*;

pub const ID: u32 = 6917479;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAMPIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change control of 1 "Vampire" monster and 1 opponent's monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register when destroying a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon monsters destroyed by battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ctfilter2(c)`
fn ctfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAMPIRE) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAMPIRE as u16]) && card::is_able_to_change_controler(d, c)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.ctfilter2,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| ctfilter2(d, c)), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g2=Duel.SelectTarget(tp,s.ctfilter2,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| ctfilter2(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g1), 2, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local a=g:GetFirst()
    // local b=g:GetNext()
    let order = group::iter(d, &g);
    let first = order.first().copied().nil();
    let second = order.get(1).copied();
    // if a:IsRelateToEffect(e) and b:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, first, a.e) && card::is_relate_to_effect(d, second.nil(), a.e) {
        // Duel.SwapControl(a,b)
        duel::swap_control(d, first, second.nil(), 0, 0);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
}
/// `s.spfilter(c,e,tp,rc,tid)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: CardId, tid: u32) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetReasonCard()==rc and c:GetTurnID()==tid
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_reason(d, c, REASON_BATTLE)
        && card::get_reason_card(d, c) == Some(rc)
        && card::get_turn_id(d, c) == tid
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tid = duel::get_turn_count(d, None) as u32;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp,e:GetHandler(),Duel.GetTurnCount()) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, h, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, h, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let tid = duel::get_turn_count(d, None) as u32;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=nil
    // local tg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, h, tid));
    let tg = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #tg>ft then
    let g = if tg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // g=tg:Select(tp,ft,ft,nil)
        group::select(d, &tg, a.tp, ft as u16, ft as u16, false, NO_EX).nil()
    } else {
        // g=tg
        tg
    };
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
