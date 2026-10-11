//! Miracle Jurassic Egg
//! 奇跡のジュラシック・エッグ
use super::prelude::*;

pub const ID: u32 = 63259351;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x14)
    card::enable_counter_permit(d, c, 0x14, None, None);
    // cannot remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e1, EFFECT_CANNOT_REMOVE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.rmlimit)
    effect::set_target(d, e2, Some(lua_fn(rmlimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // add counter
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(s.spcost)
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.rmlimit(e,c,p)`
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler()==c
    let h = effect::get_handler(d, a.e(0)).nil();
    Some(h) == a.card(1)
}
/// `s.ctfilter(c,tp)`
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsRace(RACE_DINOSAUR)
    card::is_controler(d, c, tp) && card::is_race(d, c, RACE_DINOSAUR)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if eg:IsExists(s.ctfilter,1,nil,tp) then
    if group::is_exists(d, &a.eg.clone().nil(), &|d, c| ctfilter(d, c, tp), 1, NO_EX) {
        // e:GetHandler():AddCounter(0x14,2)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0x14, 2, false);
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReleasable() end
    if a.chk == Some(0) {
        return card::is_releasable(d, h, None, None);
    }
    // e:SetLabel(e:GetHandler():GetCounter(0x14))
    let ct = card::get_counter(d, h, 0x14) as i64;
    effect::set_label(d, a.e, &[ct]);
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, h, REASON_COST, None);
    true
}
/// `s.spfilter(c,lv,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, lv: i64, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(lv) and c:IsRace(RACE_DINOSAUR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, lv as u32)
        && card::is_race(d, c, RACE_DINOSAUR)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e:GetHandler():GetCounter(0x14),e,tp) end
    if a.chk == Some(0) {
        let ct = card::get_counter(d, h, 0x14) as i64;
        let e = a.e;
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, ct, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel(),e,tp)
    let lv = effect::get_label(d, a.e);
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, lv, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
