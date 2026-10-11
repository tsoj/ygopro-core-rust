//! Invoked Dee Anima
//! 召喚獣ディー・アニマ
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 80635735;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALEISTER],
    material_setcode: &[SET_ALEISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Aleister" monster + 1 monster Special Summoned from the Extra Deck
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALEISTER),s.exdmatfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ALEISTER as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| exdmatfilter(d, c))),
        ],
    );
    // If this card is Special Summoned: You can activate 1 of these effects;
    // ● Destroy all monsters your opponent controls that were Special Summoned from the Extra Deck
    // ● Destroy all Spells/Traps your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a card effect in the GY (Quick Effect): You can banish this card; Special Summon 1 "Aleister" monster from your Deck, and if you do, negate that effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp,chk)
    //     return ep==1-tp and Chain.IsTriggeringLocation(ev,LOCATION_GRAVE) and Duel.IsChainDisablable(ev)
    // end)
    effect::set_condition(d, e2, Some(condition(disable_condition)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.exdmatfilter(c)
fn exdmatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA) and c:IsLocation(LOCATION_MZONE)
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_location(d, c, LOCATION_MZONE)
}
// The condition of e2: function(e,tp,eg,ep,ev,re,r,rp,chk)
fn disable_condition(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Chain.IsTriggeringLocation(ev,LOCATION_GRAVE) and Duel.IsChainDisablable(ev)
    a.ep == 1 - a.tp
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_GRAVE as i64]).truthy()
        && duel::is_chain_disablable(d, a.ev as u8)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(Card.IsSummonLocation,tp,0,LOCATION_MZONE,nil,LOCATION_EXTRA)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0
    let b2 = !g2.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // local g=(cd.choice==1 and g1 or g2)
    let g = if choice == Some(1) { g1 } else { g2 };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// The chosen option of `Duel.SelectEffect` as the value stored in `cd.choice` (nil when none)
fn choice_value(choice: Option<usize>) -> Value {
    match choice {
        Some(n) => Value::Int(n as i64),
        None => Value::Nil,
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // if cd.choice==1 then
    match chain::effect_data(d, a.e, None, chain::field::CARD) {
        Value::Int(1) => {
            // local g=Duel.GetMatchingGroup(Card.IsSummonLocation,tp,0,LOCATION_MZONE,nil,LOCATION_EXTRA)
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), a.tp, 0, LOCATION_MZONE, NO_EX);
            // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
            if !g.is_empty() {
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
        Value::Int(2) => {
            // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
            // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
            if !g.is_empty() {
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
        _ => {}
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ALEISTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ALEISTER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
