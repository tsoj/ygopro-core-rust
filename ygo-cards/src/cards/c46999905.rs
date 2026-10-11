//! Saambell the Star Bonder
use super::prelude::*;

pub const ID: u32 = 46999905;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Destroy and Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.destg(e,tp,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // local tc=(g-c):GetFirst()
    let mut gc = g.clone();
    group::remove_card(&mut gc, &CardSet::from_card(c));
    let tc = group::get_first(d, &gc);
    // if chk==0 then return tc and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:GetOriginalLevel()==tc:GetOriginalLevel() end
    if a.chk == Some(0) {
        let Some(tc) = tc else { return false };
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && card::get_original_level(d, c) == card::get_original_level(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,LOCATION_PZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, LOCATION_PZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_PZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_PZONE as i32);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // local tc=(g-c):GetFirst()
    let mut gc = g.clone();
    group::remove_card(&mut gc, &CardSet::from_card(c));
    let tc = group::get_first(d, &gc);
    // if not e:GetHandler():IsRelateToEffect(e) or not tc or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let Some(tc) = tc else { return };
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // if Duel.Destroy(tc,REASON_EFFECT)>0 then
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not eg:IsContains(c) and eg:IsExists(Card.IsControler,1,c,tp) then return true end
    let eg = a.eg.clone().nil();
    if !eg.contains(c) && group::is_exists(d, &eg, &|d, x| card::is_controler(d, x, a.tp), 1, c.into()) {
        return true;
    }
    false
}
/// `s.spfilter(c,e,tp,lvl)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lvl: u32) -> bool {
    // return c:IsLevel(lvl) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[lvl]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,e:GetHandler():GetLevel())
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            let h = effect::get_handler(d, a.e).nil();
            let lvl = card::get_level(d, h);
            let e = a.e;
            let tp = a.tp;
            return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, lvl)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
        }
        return false;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,c:GetLevel())
    let lvl = card::get_level(d, c);
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, e, tp, lvl)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
