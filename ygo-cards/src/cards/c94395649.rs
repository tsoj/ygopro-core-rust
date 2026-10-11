//! Cocatorium the Heavy Metal Avian
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 94395649;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Change stats
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(copycost)));
    effect::set_operation(d, e2, Some(operation(copyop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,ft,tp)`
fn cfilter(d: &mut Duel, c: CardId, ft: i64, tp: u8) -> bool {
    // return c:IsMonster() and c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
    //     and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    card::is_monster(d, c)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if chk==0 then return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,ft,tp) end
    if a.chk == Some(0) {
        return ft > -1 && duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,ft,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,c:GetLocation())
    let loc = card::get_location(d, c) as i32;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, loc);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
/// `s.costfilter(c,code,lv,race,att)`
fn costfilter(d: &mut Duel, c: CardId, code: u32, lv: u32, race: u64, att: u32) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and c:IsLevelBelow(4) and c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
    //     and (not c:IsCode(code) or not c:IsLevel(lv) or not c:IsRace(race) or not c:IsAttribute(att))
    card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && (!card::is_code(d, c, &[code]) || !card::is_level(d, c, &[lv]) || !card::is_race(d, c, race) || !card::is_attribute(d, c, att))
}
/// `s.copycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // local code, lv, race, att of c
    let code = card::get_code(d, c).first().copied().unwrap_or(0);
    let lv = card::get_level(d, c);
    let race = card::get_race(d, c);
    let att = card::get_attribute(d, c);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode(),c:GetLevel(),c:GetRace(),c:GetAttribute()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| costfilter(d, cc, code, lv, race, att)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_DECK,0,1,1,nil,c:GetCode(),c:GetLevel(),c:GetRace(),c:GetAttribute())
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| costfilter(d, cc, code, lv, race, att)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, Value::from(first));
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card();
    // if c:IsRelateToEffect(e) and not c:IsFacedown() and tc then
    if card::is_relate_to_effect(d, c, e) && !card::is_facedown(d, c) {
        if let Some(tc) = tc {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(tc:GetCode())
            let code = card::get_code(d, tc).first().copied().unwrap_or(0);
            effect::set_value(d, e1, code as i64);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
            // e2:SetValue(tc:GetLevel())
            let lv = card::get_level(d, tc);
            effect::set_value(d, e2, lv as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // e3:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e3, EFFECT_CHANGE_RACE);
            // e3:SetValue(tc:GetRace())
            let race = card::get_race(d, tc);
            effect::set_value(d, e3, race as i64);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
            // local e4=e1:Clone()
            let e4 = effect::clone(d, e1);
            // e4:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e4, EFFECT_CHANGE_ATTRIBUTE);
            // e4:SetValue(tc:GetAttribute())
            let att = card::get_attribute(d, tc);
            effect::set_value(d, e4, att as i64);
            // c:RegisterEffect(e4)
            card::register_effect(d, c, e4, false);
        }
    }
}
