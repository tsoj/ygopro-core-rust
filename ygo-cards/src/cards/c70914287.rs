//! Kuribabylon
use super::prelude::*;

pub const ID: u32 = 70914287;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[44632120, 71036835, 7021574, 34419588, CARD_KURIBOH],
    listed_series: &[SET_KURIBOH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // SP Summon Self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
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
    // ATK/DEF Up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| val(d, c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // SP Summon Kuribohs
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)>Duel.GetMatchingGroupCount(Card.IsMonster,1-tp,LOCATION_GRAVE,0,nil)
    let own = duel::get_matching_group_count(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, NO_EX);
    let opp = duel::get_matching_group_count(d, Some(&card::is_monster), 1 - a.tp, LOCATION_GRAVE, 0, NO_EX);
    own > opp
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KURIBOH) and c:IsMonster() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_KURIBOH as u16]) && card::is_monster(d, c) && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
}
// function s.val(e,c)
fn val(d: &mut Duel, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.atkfilter,c:GetControler(),LOCATION_MZONE|LOCATION_GRAVE,0,nil)*300
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&atkfilter), p, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsBattlePhase() and Duel.GetCurrentChain()==0
    duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) && duel::get_current_chain(d, false) == 0
}
// function s.spfilter(c,e,tp,code)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local locs=LOCATION_GRAVE|LOCATION_HAND
    let locs = LOCATION_GRAVE | LOCATION_HAND;
    // if chk==0 then
    if a.chk == Some(0) {
        // return c:IsAbleToHand() and Duel.GetMZoneCount(tp,c)>=5
        //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,locs,0,1,nil,e,tp,44632120)
        //     and ... (71036835, 7021574, 34419588, CARD_KURIBOH)
        return card::is_able_to_hand(d, c, None)
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 >= 5
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, 44632120)), tp, locs, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, 71036835)), tp, locs, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, 7021574)), tp, locs, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, 34419588)), tp, locs, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, CARD_KURIBOH)), tp, locs, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,5,tp,locs)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 5, tp, locs as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<5 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 5
            || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        {
            return;
        }
        // local locs=LOCATION_GRAVE|LOCATION_HAND
        let locs = LOCATION_GRAVE | LOCATION_HAND;
        // local g1=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,locs,0,nil,e,tp,44632120)
        let g1 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, 44632120))), tp, locs, 0, NO_EX);
        // local g2 ... (71036835)
        let g2 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, 71036835))), tp, locs, 0, NO_EX);
        // local g3 ... (7021574)
        let g3 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, 7021574))), tp, locs, 0, NO_EX);
        // local g4 ... (34419588)
        let g4 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, 34419588))), tp, locs, 0, NO_EX);
        // local g5 ... (CARD_KURIBOH)
        let g5 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, CARD_KURIBOH))), tp, locs, 0, NO_EX);
        // if #g1>0 and #g2>0 and #g3>0 and #g4>0 and #g5>0 then
        if !g1.is_empty() && !g2.is_empty() && !g3.is_empty() && !g4.is_empty() && !g5.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg1=g1:Select(tp,1,1,nil)
            let mut sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg2=g2:Select(tp,1,1,nil)
            let sg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
            // sg1:Merge(sg2)
            group::add_card(&mut sg1, &sg2);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg3=g3:Select(tp,1,1,nil)
            let sg3 = group::select(d, &g3, tp, 1, 1, false, NO_EX).nil();
            // sg1:Merge(sg3)
            group::add_card(&mut sg1, &sg3);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg4=g4:Select(tp,1,1,nil)
            let sg4 = group::select(d, &g4, tp, 1, 1, false, NO_EX).nil();
            // sg1:Merge(sg4)
            group::add_card(&mut sg1, &sg4);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg5=g5:Select(tp,1,1,nil)
            let sg5 = group::select(d, &g5, tp, 1, 1, false, NO_EX).nil();
            // sg1:Merge(sg5)
            group::add_card(&mut sg1, &sg5);
            // Duel.SpecialSummon(sg1,0,tp,tp,false,false,POS_FACEUP_ATTACK)
            duel::special_summon(d, sg1, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
}
