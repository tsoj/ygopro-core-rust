//! 集いし願い
//! Converging Wishes
use super::prelude::*;

pub const ID: u32 = 20007374;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, lua_fn(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // chain attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.cacon)
    effect::set_condition(d, e4, Some(condition(cacon)));
    // e4:SetCost(s.cacost)
    effect::set_cost(d, e4, Some(cost_target(cacost)));
    // e4:SetTarget(s.catg)
    effect::set_target(d, e4, Some(cost_target(catg)));
    // e4:SetOperation(s.caop)
    effect::set_operation(d, e4, Some(operation(caop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsRace(RACE_DRAGON)
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>=5
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) >= 5
}
/// `s.filter(c,e,tp,rp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // return c:IsCode(CARD_STARDUST_DRAGON) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    card::is_code(d, c, &[CARD_STARDUST_DRAGON])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp, a.rp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, a.e, tp, a.rp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP) then
    let summoned = match tc {
        Some(tc) => duel::special_summon_step(d, tc, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None),
        None => false,
    };
    if let (Some(tc), true) = (tc, summoned) {
        // if c:IsRelateToEffect(e) and not c:IsStatus(STATUS_LEAVE_CONFIRMED) then
        if card::is_relate_to_effect(d, c, a.e) && !card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
            // Duel.Equip(tp,c,tc)
            duel::equip(d, tp, c, tc, true, false);
            // --Add Equip limit
            // local e1=Effect.CreateEffect(tc)
            let e1 = effect::create_effect(d, tc);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            // (e:GetOwner()==c: the owner of e1 is tc)
            effect::set_value_fn(d, e1, lua_fn(move |_d, x: Args| x.card(1) == Some(tc)));
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(fid)
        effect::set_label(d, e2, &[fid as i64]);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetCondition(s.rmcon)
        effect::set_condition(d, e2, Some(condition(rmcon)));
        // e2:SetOperation(s.rmop)
        effect::set_operation(d, e2, Some(operation(rmop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
/// `s.atkfilter(c)` (`s.cfilter(c) and c:GetAttack()>0`)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    cfilter(d, c) && card::get_attack(d, c) > 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // local g=Duel.GetMatchingGroup(s.atkfilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&atkfilter), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetSum(Card.GetAttack)
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    let labels = card::get_flag_effect_label(d, tc, ID);
    if labels.first().copied() != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    } else {
        // return true
        true
    }
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.cacon(e,tp,eg,ep,ev,re,r,rp)`
fn cacon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        // return ec and eg:IsContains(ec)
        Some(ec) => group::is_contains(&a.eg.clone().nil(), ec),
        None => false,
    }
}
/// `s.cafilter(c)`
fn cafilter(d: &mut Duel, c: CardId) -> bool {
    // return s.cfilter(c) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    cfilter(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.cacost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cacost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cafilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler():GetEquipTarget()) end
    if a.chk == Some(0) {
        let ec = card::get_equip_target(d, h);
        return duel::is_existing_matching_card(d, Some(&cafilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, ec.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cafilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler():GetEquipTarget())
    let ec = card::get_equip_target(d, h);
    let g = duel::select_matching_card(d, tp, Some(&cafilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, ec.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.catg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn catg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return ec:CanChainAttack(0,true) end
    if a.chk == Some(0) {
        return card::can_chain_attack(d, ec.nil(), Some(0), true);
    }
    true
}
/// `s.caop(e,tp,eg,ep,ev,re,r,rp)`
fn caop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    // if not ec or not ec:IsRelateToBattle() then return end
    let ec = match card::get_equip_target(d, c) {
        Some(ec) if card::is_relate_to_battle(d, ec) => ec,
        _ => return,
    };
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
    // ec:RegisterEffect(e1)
    card::register_effect(d, ec, e1, false);
}
