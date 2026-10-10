//! Subterror Nemesis Warrior
use super::prelude::*;

pub const ID: u32 = 16719140;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUBTERROR, SET_SUBTERROR_BEHEMOTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute this card and at least 1 other monster and Special Summon 1 "Subterror" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.subspcost)
    effect::set_cost(d, e1, Some(cost_target(subspcost)));
    // e1:SetTarget(s.subsptg)
    effect::set_target(d, e1, Some(cost_target(subsptg)));
    // e1:SetOperation(s.subspop)
    effect::set_operation(d, e1, Some(operation(subspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_FLIP)
    effect::set_code(d, e2, EVENT_FLIP);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfspcon)
    effect::set_condition(d, e2, Some(condition(selfspcon)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.subspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn subspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    true
}
/// `s.costfilter(c,e,tp,mg,rlv)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mg: &CardSet, rlv: u32) -> bool {
    // if not (c:HasLevel() and c:IsSetCard(SET_SUBTERROR) and c:IsMonster() and c:IsAbleToGraveAsCost()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_DEFENSE)) then return false end
    if !(card::has_level(d, c)
        && card::is_set_card(d, c, &[SET_SUBTERROR as u16])
        && card::is_monster(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_DEFENSE as u8), None, None))
    {
        return false;
    }
    // local lv=c:GetLevel()-rlv
    let lv = card::get_level(d, c) as i64 - rlv as i64;
    // return #mg>0 and (lv<=0 or mg:CheckWithSumGreater(Card.GetOriginalLevel,lv))
    !mg.is_empty() && (lv <= 0 || group::check_with_sum_greater(d, mg, &|d, x| card::get_original_level(d, x) as i64, lv as u32).0)
}
/// `s.subsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn subsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=Duel.GetReleaseGroup(tp,false,false,REASON_EFFECT):Filter(Card.IsLevelAbove,nil,1)
    let rg = duel::get_release_group(d, a.tp, false, false, Some(REASON_EFFECT));
    let mut mg = group::filter(d, &rg, &|d, x| card::is_level_above(d, x, 1), NO_EX);
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // if not mg:IsContains(c) then return false end
        if !mg.contains(c) {
            return false;
        }
        // mg:RemoveCard(c)
        group::remove_card(&mut mg, &CardSet::from_card(c));
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2
        //     and Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_DECK,0,1,nil,e,tp,mg,c:GetOriginalLevel())
        let lvl = card::get_original_level(d, c);
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -2
            && duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, a.e, a.tp, &mg, lvl)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // mg:RemoveCard(c)
    group::remove_card(&mut mg, &CardSet::from_card(c));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mg,c:GetOriginalLevel())
    let lvl = card::get_original_level(d, c);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| costfilter(d, x, a.e, a.tp, &mg, lvl)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.subspop(e,tp,eg,ep,ev,re,r,rp)`
fn subspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<-1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < -1 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local mg=Duel.GetReleaseGroup(tp,false,false,REASON_EFFECT):Filter(Card.IsLevelAbove,nil,1)
    let rg = duel::get_release_group(d, a.tp, false, false, Some(REASON_EFFECT));
    let mut mg = group::filter(d, &rg, &|d, x| card::is_level_above(d, x, 1), NO_EX);
    // if not mg:IsContains(c) then return end
    if !mg.contains(c) {
        return;
    }
    // mg:RemoveCard(c)
    group::remove_card(&mut mg, &CardSet::from_card(c));
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // if tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_DEFENSE) then
    if card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_DEFENSE as u8), None, None) {
        // local lv=tc:GetLevel()-c:GetOriginalLevel()
        let lv = card::get_level(d, tc) as i64 - card::get_original_level(d, c) as i64;
        // local g=Group.CreateGroup()
        // if lv<=0 then ... g=mg:Select(tp,1,1,nil) else ... g=mg:SelectWithSumGreater(tp,Card.GetOriginalLevel,lv) end
        let mut g = if lv <= 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
            group::select(d, &mg, a.tp, 1, 1, false, NO_EX).nil()
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
            group::select_with_sum_greater(d, &mg, a.tp, &|d, x| card::get_original_level(d, x) as i64, lv as u32).nil()
        };
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // if #g>=2 and Duel.Release(g,REASON_EFFECT)~=0 then
        if g.len() >= 2 && duel::release(d, g.clone(), REASON_EFFECT, None) != 0 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_DEFENSE)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_DEFENSE as u8, None);
        }
    }
}
/// `s.selfspconfilter(c,tp)`
fn selfspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUBTERROR_BEHEMOTH) and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_SUBTERROR_BEHEMOTH as u16]) && card::is_controler(d, c, tp)
}
/// `s.selfspcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.selfspconfilter,1,nil,tp)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| selfspconfilter(d, c, tp), 1, NO_EX)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
