//! Luminous Dragon Ritual
//! 光子竜降臨
use super::prelude::*;

pub const ID: u32 = 34834619;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[85346853], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon 1 "Paladin of Photon Dragon" from your hand
    // Ritual.AddProcEqualCode(c,4,nil,85346853)
    ritual::add_proc_equal_code(d, c, Some(ritual::Lv::N(4)), None, &[85346853]);
    // Banish monsters from your GY whose total Levels equal exactly 4, then Special Summon 1 "Paladin of Photon Dragon" from your hand (this is treated as a Ritual Summon)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c,e,tp)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:HasLevel() and c:IsAbleToRemove() and not c:IsImmuneToEffect(e) and aux.SpElimFilter(c,true)
    card::has_level(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && !card::is_immune_to_effect(d, c, e)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:GetSum(Card.GetLevel)==4
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) == 4;
    (res, false)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(85346853) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,true,false)
    card::is_code(d, c, &[85346853]) && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e,tp)
        let rg = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, a.e)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
        // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,4,s.rescon,0)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        return !rg.is_empty()
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(4), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            )
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, a.e)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,4,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(1),
        Some(4),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #rg>0 and Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and rg:IsExists(Card.IsLocation,1,nil,LOCATION_REMOVED) then
    if !rg.is_empty()
        && duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_REMOVED), 1, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummon(sc,SUMMON_TYPE_RITUAL,tp,tp,true,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, SUMMON_TYPE_RITUAL, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
            }
        }
    }
}
