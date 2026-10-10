//! Vampire Voivode
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 4918855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If normal summoned, special summon up to 2 monsters from opponent's GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of a monster effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(function(_,_,_,_,ev)Duel.NegateActivation(ev)end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon itself from GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCost(s.sscost)
    effect::set_cost(d, e3, Some(cost_target(sscost)));
    // e3:SetCondition(s.sscon)
    effect::set_condition(d, e3, Some(condition(sscon)));
    // e3:SetTarget(s.sstg)
    effect::set_target(d, e3, Some(cost_target(sstg)));
    // e3:SetOperation(s.ssop)
    effect::set_operation(d, e3, Some(operation(ssop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`: Check for a monster to special summon
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`: Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ct>2 then ct=2 end
    if ct > 2 {
        ct = 2;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,0,LOCATION_GRAVE,1,ct,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`: Special summon up to 2 monsters from opponent's GY
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    let _first = duel::get_target_cards(d, e);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if #g>ct or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then
    if g.len() as i32 > ct || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // g=g:Select(tp,1,1,nil)
        g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    }
    // local tc=g:GetFirst()
    let _tc0 = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Negate effect(s)
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`: If a monster effect is activated
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,re:GetHandler():GetCode())
    let re = a.re.nil();
    if !(effect::is_monster_effect(d, re) && duel::is_chain_negatable(d, a.ev as u8)) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    let codes = card::get_code(d, rh);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &codes)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`: Activation legality
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.cfilter(c,tp)`: Check if a monster was special summoned from opponent's GY
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonLocation(LOCATION_GRAVE) and c:IsPreviousControler(1-tp) and c:IsOriginalType(TYPE_MONSTER)
    card::is_summon_location(d, c, LOCATION_GRAVE) && card::is_previous_controler(d, c, 1 - tp) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.sscon(e,tp,eg,ep,ev,re,r,rp)`: If it ever happened
fn sscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.sscost(e,tp,eg,ep,ev,re,r,rp,chk)`: Tribute 2 monsters as cost
fn sscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,false,aux.ReleaseCheckMMZ,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, None, 2, 2, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,aux.ReleaseCheckMMZ,nil)
    let g = duel::select_release_group_cost(d, tp, None, 2, 2, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sstg(e,tp,eg,ep,ev,re,r,rp,chk)`: Activation legality
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.ssop(e,tp,eg,ep,ev,re,r,rp)`: Special summon itself from GY
fn ssop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
