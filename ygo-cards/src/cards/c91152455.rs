//! Flame Coating Metalmorph
use super::prelude::*;

pub const ID: u32 = 91152455;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.self_equip_trap=true
    self_equip_trap: true,
    // s.listed_names={CARD_MAX_METALMORPH}
    listed_names: &[CARD_MAX_METALMORPH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster that cannot be Normal Summoned/Set and mentions "Max Metalmorph" from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:ListsCode(CARD_MAX_METALMORPH) and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    card::lists_code(d, c, &[CARD_MAX_METALMORPH]) && card::is_faceup(d, c) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,tp)
        let res = duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, tp)), 1, 1, false, None, NO_EX);
        // if res then e:SetLabel(1) end
        if res {
            effect::set_label(d, e, &[1]);
        }
        // return res
        return res;
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,tp):GetFirst()
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &g).nil();
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:ListsCode(CARD_MAX_METALMORPH) and c:IsMonster() and not c:IsSummonableCard()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::lists_code(d, c, &[CARD_MAX_METALMORPH]) && card::is_monster(d, c) && !card::is_summonable_card(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        //     and (e:GetLabel()==1 or Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
        let res = duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && (effect::get_label(d, e) == 1 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0);
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) > 0 {
            // if not (c:IsRelateToEffect(e) and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
            if !(card::is_relate_to_effect(d, c, e) && duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
                return;
            }
            // c:CancelToGrave(true)
            card::cancel_to_grave(d, c, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if not tc:EquipByEffectAndLimitRegister(e,tp,c,nil,true) then return end
            if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
                return;
            }
            // --Equip limit
            // local e0=Effect.CreateEffect(c)
            let e0 = effect::create_effect(d, c);
            // e0:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
            // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e0:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
            // e0:SetValue(function(e,c) return c==tc end)
            effect::set_value_fn(d, e0, lua_fn(move |_d, a: Args| a.c(1) == tc));
            // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e0)
            card::register_effect(d, c, e0, false);
            // --If the equipped monster would be destroyed by battle or card effect, you can send this card to the GY instead
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_EQUIP+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EFFECT_DESTROY_REPLACE)
            effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
            // e1:SetTarget(s.desreptg)
            effect::set_target(d, e1, Some(cost_target(desreptg)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    // if chk==0 then return ec:IsReason(REASON_BATTLE|REASON_EFFECT) and not ec:IsReason(REASON_REPLACE)
    //     and c:IsAbleToGrave() and not c:IsStatus(STATUS_DESTROY_CONFIRMED) end
    if a.chk == Some(0) {
        let ec = card::get_equip_target(d, c).nil();
        return card::is_reason(d, ec, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, ec, REASON_REPLACE) && card::is_able_to_grave(d, c, None) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and Duel.SendtoGrave(c,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, tp, c, Some(96)) && duel::send_to_grave(d, c, REASON_EFFECT, None, None) > 0
}
