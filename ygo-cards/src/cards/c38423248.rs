//! Invoked Magistus Omega
//! 召喚獣マギストス・セリオン
use super::prelude::*;

pub const ID: u32 = 38423248;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86120751], miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Aleister the Invoker" + 1 Fusion, Synchro, Xyz, or Link Monster
    // Fusion.AddProcMix(c,true,true,86120751,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EXTRA))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(86120751),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_EXTRA, Some(fc), st, Some(tp)))),
        ],
    );
    // Banish 1 Fusion, Synchro, Xyz, or Link Monster from your GY and 1 monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 4 Spellcaster monster from your Deck, then you can equip this card to it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
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
/// `s.rmfilter(c,e)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsAbleToRemove() and (c:IsType(TYPE_EXTRA) or c:IsLocation(LOCATION_MZONE))
    card::is_can_be_effect_target(d, c, Some(e)) && card::is_able_to_remove(d, c, None, None, None) && (card::is_type(d, c, TYPE_EXTRA) || card::is_location(d, c, LOCATION_MZONE))
}
/// `s.rmrescon(sg,e,tp,mg)`
fn rmrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==1
    (group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) == 1, false)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, a.e)), a.tp, LOCATION_GRAVE | LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rmrescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rmrescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rmrescon,1,tp,HINTMSG_REMOVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rmrescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg), 2, a.tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e):Match(Card.IsMonster,nil)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_monster(d, c), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_fusion_summoned(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(4) and c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e);
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, h.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
            && card::is_relate_to_effect(d, c, a.e)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Equip(tp,c,sc) then
            if duel::equip(d, a.tp, c, sc, true, false) {
                // local e0=Effect.CreateEffect(c)
                let e0 = effect::create_effect(d, c);
                // e0:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
                // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e0:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
                // e0:SetValue(function(e,c) return c==sc end)
                effect::set_value_fn(d, e0, effect_card_fn(move |_d, _e, cc| cc == sc));
                // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
                // c:RegisterEffect(e0)
                card::register_effect(d, c, e0, false);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(1000)
                effect::set_value(d, e1, 1000);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
