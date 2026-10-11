//! Gouki Sheik Ogre
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 7382007;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Link Summon procedure: 2 "Gouki" monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2,2)
    link::add_procedure(d, c, Some(fusion_mat_gouki()), 2, Some(2), None, None, None);
    // --Special Summon any number of "Gouki" monsters from your hand with different Levels from each other
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Fusion Summon 1 "Gouki" Fusion Monster from your Extra Deck, using monsters from your hand or field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(Fusion.SummonEffTG(aux.FilterBoolFunction(Card.IsSetCard,SET_GOUKI)))
    let tg = fusion::summon_eff_tg(d, fus_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(aux.FilterBoolFunction(Card.IsSetCard,SET_GOUKI)))
    let op = fusion::summon_eff_op(d, fus_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI)` as a Link material filter
fn fusion_mat_gouki() -> MatFilter {
    mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GOUKI as u16], sc, st, Some(tp)))
}
/// `aux.FilterBoolFunction(Card.IsSetCard,SET_GOUKI)` as the Fusion filter of `Fusion.SummonEffTG/OP`
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_GOUKI as u16]))),
        ..Default::default()
    }
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsMonster() and c:IsPreviousLocation(LOCATION_DECK)
    //   and c:IsControler(tp) and not c:IsReason(REASON_DRAW)
    card::is_set_card(d, c, &[SET_GOUKI as u16])
        && card::is_monster(d, c)
        && card::is_previous_location(d, c, LOCATION_DECK)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_DRAW)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| spconfilter(d, x, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `local ft ...` helper: the level of a card as a number (`Card.GetLevel`)
fn level_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // ft=math.min(ft,#g)
    ft = ft.min(g.len() as i32);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dpcheck(Card.GetLevel),1,tp,HINTMSG_SPSUMMON)
    let dp = aux::dpcheck(level_of);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&dp), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
            // local e1=Effect.CreateEffect(e:GetHandler())
            let c = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e1, tp, 1, 0);
            // e1:SetCondition(function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end)
            effect::set_condition(d, e1, Some(condition(|d, a| {
                let h = effect::get_handler(d, a.e).nil();
                let owner = effect::get_owner_player(d, a.e);
                card::is_controler(d, h, owner)
            })));
            // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_GOUKI) end)
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, x| !card::is_set_card(d, x, &[SET_GOUKI as u16]))));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
        }
    }
}
