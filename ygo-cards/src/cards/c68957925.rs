//! Link Party
use super::prelude::*;

pub const ID: u32 = 68957925;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_RECOVER+CATEGORY_DAMAGE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_RECOVER + CATEGORY_DAMAGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttackAbove(2500) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attack_above(d, c, 2500) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackBelow(3000)
    card::is_faceup(d, c) && card::is_attack_below(d, c, 3000)
}
/// The face-up Link Monsters on the field (`aux.FaceupFilter(Card.IsLinkMonster)` over both sides).
fn link_faceup(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_link_monster(d, c))), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLinkMonster),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let lg = link_faceup(d, tp);
    // local ct=lg:GetClassCount(Card.GetOriginalAttribute)
    let ct = group::get_class_count(d, &lg, &|d, c| card::get_original_attribute(d, c) as i64);
    // if chk==0 then
    if a.chk == Some(0) {
        // return ct==3 or ct==4
        //     or (ct==1 and lg:FilterCount(Card.IsControler,nil,tp)>0)
        //     or (ct==2 and lg:FilterCount(Card.IsControler,nil,1-tp)>0)
        //     or (ct==5 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp))
        //     or (ct==6 and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil))
        return ct == 3
            || ct == 4
            || (ct == 1 && group::filter_count(d, &lg, &|d, c| card::is_controler(d, c, tp), NO_EX) > 0)
            || (ct == 2 && group::filter_count(d, &lg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX) > 0)
            || (ct == 5
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX))
            || (ct == 6 && duel::is_existing_matching_card(d, Some(&desfilter), tp, 0, LOCATION_MZONE, 1, NO_EX));
    }
    // if ct==3 then
    if ct == 3 {
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,1500)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 1500);
    // elseif ct==4 then
    } else if ct == 4 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 2000);
    // elseif ct==5 then
    } else if ct == 5 {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // elseif ct==6 then
    } else if ct == 6 {
        // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local lg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLinkMonster),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let lg = link_faceup(d, tp);
    // if #lg==0 then return end
    if lg.is_empty() {
        return;
    }
    // local ct=lg:GetClassCount(Card.GetOriginalAttribute)
    let ct = group::get_class_count(d, &lg, &|d, c| card::get_original_attribute(d, c) as i64);
    // if ct==1 then
    if ct == 1 {
        // local g1=lg:Filter(Card.IsControler,nil,tp)
        let g1 = group::filter(d, &lg, &|d, c| card::is_controler(d, c, tp), NO_EX);
        // if #g1==0 then return end
        if g1.is_empty() {
            return;
        }
        // for tc1 in g1:Iter() do
        for tc1 in group::iter(d, &g1) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(500)
            effect::set_value(d, e1, 500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            // tc1:RegisterEffect(e1)
            card::register_effect(d, tc1, e1, false);
        }
    // elseif ct==2 then
    } else if ct == 2 {
        // local g2=lg:Filter(Card.IsControler,nil,1-tp)
        let g2 = group::filter(d, &lg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
        // if #g2==0 then return end
        if g2.is_empty() {
            return;
        }
        // for tc2 in g2:Iter() do
        for tc2 in group::iter(d, &g2) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-1000)
            effect::set_value(d, e1, -1000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            // tc2:RegisterEffect(e1)
            card::register_effect(d, tc2, e1, false);
        }
    // elseif ct==3 then
    } else if ct == 3 {
        // Duel.Recover(tp,1500,REASON_EFFECT)
        duel::recover(d, tp, 1500, REASON_EFFECT, false, None);
    // elseif ct==4 then
    } else if ct == 4 {
        // Duel.Damage(1-tp,2000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 2000, REASON_EFFECT, false, None);
    // elseif ct==5 then
    } else if ct == 5 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
            return;
        }
        // local g3=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g3 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // if #g3==0 then return end
        if g3.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g3:Select(tp,1,1,nil)
        let sg = group::select(d, &g3, tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // elseif ct==6 then
    } else if ct == 6 {
        // local g4=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
        let g4 = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g4==0 then return end
        if g4.is_empty() {
            return;
        }
        // Duel.Destroy(g4,REASON_EFFECT)
        duel::destroy(d, g4, REASON_EFFECT, None, None);
    }
}
