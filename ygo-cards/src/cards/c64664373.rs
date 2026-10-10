//! Metaltronus
use super::prelude::*;

pub const ID: u32 = 64664373;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster with 2+ properties equal to the target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_TOKEN) and c:IsNegatableMonster()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,c:GetRace(),c:GetAttribute(),c:GetAttack())
    if !(card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN) && card::is_negatable_monster(d, c)) {
        return false;
    }
    let rac = card::get_race(d, c);
    let att = card::get_attribute(d, c);
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, rac, att, atk)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,rac,att,atk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rac: u64, att: u32, atk: i32) -> bool {
    // local ct=0
    let mut ct = 0;
    // if c:IsRace(rac) then ct=ct+1 end
    if card::is_race(d, c, rac) {
        ct += 1;
    }
    // if c:IsAttribute(att) then ct=ct+1 end
    if card::is_attribute(d, c, att) {
        ct += 1;
    }
    // if c:IsAttack(atk) then ct=ct+1 end
    if card::is_attack(d, c, &[atk as u32]) {
        ct += 1;
    }
    // return ct>=2 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0)
    //         or (not c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0))
    if ct < 2 || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    if card::is_location(d, c, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,0,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, a.e, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.cfilter,tp,0,LOCATION_MZONE,1,1,nil,e,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, tp, 0);
    // --Your opponent cannot activate the targeted monster's effects in response to this card's activation
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(function(e,tp,p) return e:GetHandler()~=tc end)
        duel::set_chain_limit(d, lua_fn(move |d, a: Args| effect::get_handler(d, a.e(0)) != Some(tc)));
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp,tc:GetRace(),tc:GetAttribute(),tc:GetAttack()):GetFirst()
    let rac = card::get_race(d, tc);
    let att = card::get_attribute(d, tc);
    let atk = card::get_attack(d, tc);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| spfilter(d, c, a.e, a.tp, rac, att, atk)),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // --Negate the effects of the summoned monster
            // sc:NegateEffects(c)
            card::negate_effects(d, sc, c, None, false, None);
            // if Duel.SpecialSummonComplete()==0 then return end
            if duel::special_summon_complete(d) == 0 {
                return;
            }
            // --Negate the effects of the targeted monster
            // tc:NegateEffects(c)
            card::negate_effects(d, tc, c, None, false, None);
            // Duel.AdjustInstantly(tc)
            duel::adjust_instantly(d, Some(tc));
            // if sc:IsCode(tc:GetCode()) and sc:IsAbleToRemove(tp,POS_FACEDOWN,REASON_EFFECT)
            //     and tc:IsAbleToRemove(tp,POS_FACEDOWN,REASON_EFFECT)
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            let codes = card::get_code(d, tc);
            if card::is_code(d, sc, &codes)
                && card::is_able_to_remove(d, sc, Some(a.tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))
                && card::is_able_to_remove(d, tc, Some(a.tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Remove(Group.FromCards(tc,sc),POS_FACEDOWN,REASON_EFFECT)
                duel::remove(d, group::create_group(&[tc, sc]), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
