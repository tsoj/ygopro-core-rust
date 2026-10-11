//! T.G. Halberd Cannon/Assault Mode
use super::prelude::*;

pub const ID: u32 = 47027714;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ASSAULT_MODE, 97836203], assault_mode: Some(97836203), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Negate an opponent's Summon, and if you do, banish that monster and all Special Summoned monsters your opponent controls
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1a, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e1a:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1a, EFFECT_TYPE_QUICK_O);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1a, EVENT_SUMMON);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and Duel.GetCurrentChain()==0 end)
    effect::set_condition(d, e1a, Some(condition(|d, a| a.ep == 1 - a.tp && duel::get_current_chain(d, false) == 0)));
    // e1a:SetTarget(s.negsumtg)
    effect::set_target(d, e1a, Some(cost_target(negsumtg)));
    // e1a:SetOperation(s.negsumop)
    effect::set_operation(d, e1a, Some(operation(negsumop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1c, EVENT_SPSUMMON);
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // Special Summon 1 "T.G. Halberd Cannon" from your GY, ignoring its Summoning conditions
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsAbleToRemove,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg), n, a.tp, 0);
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsAbleToRemove()
    card::is_special_summoned(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.negsumop(e,tp,eg,ep,ev,re,r,rp)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, eg.clone());
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // g:Merge(eg)
    group::add_card(&mut g, &eg);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(97836203) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[97836203]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
