//! Cubic Rebirth
use super::prelude::*;

pub const ID: u32 = 71442223;

const COUNTER_CUBIC: u16 = 0x1038;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CUBIC], counter_place_list: &[0x1038], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE+CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil end)
    effect::set_condition(d, e1, Some(condition(attack_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil end`
fn attack_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.cubicspfilter(c,e,tp)`
fn cubicspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CUBIC) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_CUBIC as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d);
    // if chkc then return chkc==bc end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == bc;
    }
    // if chk==0 then return bc:IsOnField() and bc:IsCanBeEffectTarget(e) and bc:IsCanAddCounter(COUNTER_CUBIC,1)
    //     and Duel.IsExistingMatchingCard(s.cubicspfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let bc = bc.nil();
        return card::is_on_field(d, bc)
            && card::is_can_be_effect_target(d, bc, Some(e))
            && card::is_can_add_counter(d, bc, COUNTER_CUBIC, 1, false, 0)
            && duel::is_existing_matching_card(d, Some(&|d, x| cubicspfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(bc)
    if let Some(bc) = bc {
        duel::set_target_card(d, bc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.oppspfilter(c,e,tp,code)`
fn oppspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local opp=1-tp
    let opp = 1 - tp;
    // local tg=Group.FromCards(tc)
    let mut tg = CardSet::from_card(tc);
    // local ft=Duel.GetLocationCount(opp,LOCATION_MZONE,opp)
    let mut ft = duel::get_location_count(d, opp, LOCATION_MZONE as u8, Some(opp), None, None).0;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.oppspfilter),opp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,opp,tc:GetCode())
    let code = card::get_code(d, tc)[0];
    let f = aux::necro_valley_filter(|d, x| oppspfilter(d, x, e, opp, code));
    let g = duel::get_matching_group(d, Some(&f), opp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if ft>0 and #g>0 then
    if ft > 0 && !g.is_empty() {
        // if Duel.IsPlayerAffectedByEffect(opp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, opp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(opp,ft,ft,nil)
        let sg = group::select(d, &g, opp, ft as u16, ft as u16, false, NO_EX).nil();
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // Duel.SpecialSummonStep(sc,0,opp,opp,false,false,POS_FACEUP_ATTACK)
            duel::special_summon_step(d, sc, 0, opp, opp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // tg:Merge(og)
        group::add_card(&mut tg, &og);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local counter_chk=false
    let mut counter_chk = false;
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if tc:AddCounter(COUNTER_CUBIC,1) then counter_chk=true end
        if card::add_counter(d, tc, COUNTER_CUBIC, 1, false) {
            counter_chk = true;
        }
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // e2:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_CUBIC)>0 end)
        effect::set_condition(d, e2, Some(condition(has_counter)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e3, EFFECT_DISABLE);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
    let _ = counter_chk;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local cubicg=Duel.SelectMatchingCard(tp,s.cubicspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let cubicg = duel::select_matching_card(d, tp, Some(&|d, x| cubicspfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #cubicg>0 then
    if !cubicg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(cubicg,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, cubicg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `function(e) return e:GetHandler():GetCounter(COUNTER_CUBIC)>0 end`
fn has_counter(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_CUBIC) > 0
}
