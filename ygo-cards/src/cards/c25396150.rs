//! Amazoness Hall
use super::prelude::*;

pub const ID: u32 = 25396150;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_BATTLE_START | TIMING_END_PHASE) as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain LP equal to a target's ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.lpcond)
    effect::set_condition(d, e2, Some(condition(lpcond)));
    // e2:SetTarget(s.lptg)
    effect::set_target(d, e2, Some(cost_target(lptg)));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.pendfilter(c,tp)`
fn pendfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and not c:IsForbidden() and Duel.CheckPendulumZones(tp)
    card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c) && duel::check_pendulum_zones(d, tp)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_AMAZONESS) and c:IsFaceup() and (c:IsAbleToHand() or s.pendfilter(c,tp))
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
        && card::is_faceup(d, c)
        && (card::is_able_to_hand(d, c, None) || pendfilter(d, c, tp))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.cfilter),tp,LOCATION_GRAVE|LOCATION_EXTRA,0,nil,tp)
    let f = aux::necro_valley_filter(|d, c| cfilter(d, c, tp));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local tc=g:Select(tp,1,1,nil)
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // aux.ToHandOrElse(tc,tp,function(tc) return s.pendfilter(tc,tp) end,
        //     function(tc) Duel.MoveToField(tc:GetFirst(),tp,tp,LOCATION_PZONE,POS_FACEUP,true) end,
        //     aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, c| pendfilter(d, c, tp)),
            Some(&|d, tg| {
                let first = group::get_first(d, tg).nil();
                duel::move_to_field(d, first, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    }
}
/// `s.amzfilter(c)`
fn amzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZONESS) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.lpcond(e,tp,eg,ep,ev,re,r,rp)`
fn lpcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.amzfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&amzfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.lpfilter(c,e,tp)`
fn lpfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsFaceup() and c:GetAttack()>0 and c:IsCanBeEffectTarget(e)
    //     and c:IsLocation(LOCATION_MZONE)
    card::is_controler(d, c, 1 - tp)
        && card::is_faceup(d, c)
        && card::get_attack(d, c) > 0
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.lptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.lpfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && lpfilter(d, chkc, e, tp);
    }
    // if chk==0 then return eg:IsExists(s.lpfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| lpfilter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.lpfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &eg, tp, &|d, c| lpfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,g:GetFirst():GetAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, atk);
    true
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        // local value=tc:GetAttack()
        let value = card::get_attack(d, tc);
        // if value==0 then return end
        if value == 0 {
            return;
        }
        // Duel.Recover(tp,value,REASON_EFFECT)
        duel::recover(d, tp, value as i64, REASON_EFFECT, false, None);
    }
}
