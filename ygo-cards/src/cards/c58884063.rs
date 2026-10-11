//! Gold Pride - Nytro Blaster
use super::prelude::*;

pub const ID: u32 = 58884063;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[59900655], listed_series: &[SET_GOLD_PRIDE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,s.mfilter,2,nil,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, lc, st, tp| mfilter(d, c, lc, st, tp))), 2, None, Some(group_check(matcheck)), None, None);
    // Destroy 1 monster the opponent controls and cards adjacent to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return itself to the Extra Deck and Special Summon "Gold Pride - Nytro Head"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c,lc,sumtype,tp)`
fn mfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOLD_PRIDE,lc,sumtype,tp) and c:HasLevel()
    card::is_set_card_ctx(d, c, &[SET_GOLD_PRIDE as u16], lc, sumtype, Some(tp)) && card::has_level(d, c)
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:GetClassCount(Card.GetLevel)==#g
    group::get_class_count(d, g, &|d, c| card::get_level(d, c) as i64) == g.len()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
    let tc = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, 0, 0);
    // Register that this effect was activated this turn
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_OATH,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
/// `s.cfilter(c,tc,seq)`
fn cfilter(d: &mut Duel, c: CardId, tc: CardId, seq: u32) -> bool {
    // if c:IsLocation(LOCATION_SZONE) then
    if card::is_location(d, c, LOCATION_SZONE) {
        // return tc:IsInMainMZone() and tc:GetColumnGroup():IsContains(c) and c:IsControler(tc:GetControler())
        let controler = card::get_controler(d, tc) as u8;
        card::is_in_main_m_zone(d, tc, None) && {
            let cg = card::get_column_group(d, tc, 0, 0);
            group::is_contains(&cg, c)
        } && card::is_controler(d, c, controler)
    // elseif c:IsLocation(LOCATION_MZONE) then
    } else if card::is_location(d, c, LOCATION_MZONE) {
        // if c:IsInExtraMZone() or tc:IsInExtraMZone() then
        if card::is_in_extra_m_zone(d, c, None) || card::is_in_extra_m_zone(d, tc, None) {
            // return tc:GetColumnGroup():IsContains(c)
            let cg = card::get_column_group(d, tc, 0, 0);
            group::is_contains(&cg, c)
        } else {
            // return c:IsSequence(seq-1,seq+1) and c:IsControler(tc:GetControler())
            let controler = card::get_controler(d, tc) as u8;
            card::is_sequence(d, c, &[seq.wrapping_sub(1), seq + 1]) && card::is_controler(d, c, controler)
        }
    } else {
        // return false
        false
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local g=tc:GetColumnGroup(1,1):Filter(s.cfilter,nil,tc,tc:GetSequence())
    let colg = card::get_column_group(d, tc, 1, 1);
    let seq = card::get_sequence(d, tc);
    let g = group::filter(d, &colg, &|d, c| cfilter(d, c, tc, seq), NO_EX);
    // if Duel.Destroy(tc,REASON_EFFECT)>0 and Duel.GetLP(tp)<Duel.GetLP(1-tp) and #g>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::get_lp(d, tp) < duel::get_lp(d, 1 - tp)
        && !g.is_empty()
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(59900655) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[59900655]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsAbleToExtra() and Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0
    //     and c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_able_to_extra(d, c)
        && duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_EXTRA)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
