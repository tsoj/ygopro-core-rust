//! Gouki Destroy Ogre
//! 剛鬼デストロイ・オーガ
use super::prelude::*;

pub const ID: u32 = 88406570;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GOUKI as u16], sc, st, Some(tp)))), 2, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| indtg(d, e, c))));
    // e1:SetValue(s.indct)
    effect::set_value_fn(d, e1, lua_fn(indct));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indtg(e,c)`: `return e:GetHandler():GetLinkedGroup():IsContains(c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    lg.contains(c)
}
/// `s.indct(e,re,r,rp)`: `if r&REASON_BATTLE~=0 then return 1 else return 0 end`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    let r = a.int(2) as u32;
    if r & REASON_BATTLE != 0 { 1 } else { 0 }
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c,e,tp,zone)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsSetCard(SET_GOUKI) and not c:IsLinkMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_set_card(d, c, &[SET_GOUKI as u16])
        && !card::is_link_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone()
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, None);
        // return zone~=0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter1,1-tp,LOCATION_GRAVE,0,1,nil,e,1-tp)
        //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_GRAVE,0,1,nil,e,tp,zone)
        return zone != 0
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, a.e, 1 - a.tp)), 1 - a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp)
    let mut ft = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), None, None).0;
    // if ft>0 then
    if ft > 0 {
        // if Duel.IsPlayerAffectedByEffect(1-tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, 1 - a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // if ft>1 then ft=2 end
        if ft > 1 {
            ft = 2;
        }
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
        // local g1=Duel.SelectMatchingCard(1-tp,aux.NecroValleyFilter(s.spfilter1),1-tp,LOCATION_GRAVE,0,1,ft,nil,e,1-tp)
        let g1 = duel::select_matching_card(
            d,
            1 - a.tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter1(d, c, a.e, 1 - a.tp))),
            1 - a.tp,
            LOCATION_GRAVE,
            0,
            1,
            ft as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #g1>0 then
        if !g1.is_empty() {
            // local ct=Duel.SpecialSummon(g1,0,1-tp,1-tp,false,false,POS_FACEUP)
            let mut ct = duel::special_summon(d, g1, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
            // local zone=c:GetLinkedZone(tp)
            let zone = card::get_linked_zone(d, c, Some(a.tp));
            // ct=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone),ct)
            let loc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
            ct = std::cmp::min(loc, ct);
            // if zone~=0 and ct>0 and c:IsRelateToEffect(e) then
            if zone != 0 && ct > 0 && card::is_relate_to_effect(d, c, a.e) {
                // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
                if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
                    ct = 1;
                }
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local g2=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_GRAVE,0,1,ct,nil,e,tp,zone)
                let g2 = duel::select_matching_card(
                    d,
                    a.tp,
                    Some(&aux::necro_valley_filter(|d, c| spfilter2(d, c, a.e, a.tp, zone))),
                    a.tp,
                    LOCATION_GRAVE,
                    0,
                    1,
                    ct as u16,
                    false,
                    NO_EX,
                )
                .nil();
                // if #g2>0 then
                if !g2.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(g2,0,tp,tp,false,false,POS_FACEUP,zone)
                    duel::special_summon(d, g2, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
                }
            }
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| splimit(d, e, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c)`: `return not c:IsSetCard(SET_GOUKI)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_set_card(d, c, &[SET_GOUKI as u16])
}
