//! Super Quantal Mech Ship Magnacarrier
use super::prelude::*;

pub const ID: u32 = 10424147;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPER_QUANTUM, SET_SUPER_QUANTAL_MECH_BEAST],
    listed_names: &[84025439],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_cost(d, e2, Some(cost_target(spcost1)));
    effect::set_target(d, e2, Some(cost_target(sptg1)));
    effect::set_operation(d, e2, Some(operation(spop1)));
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    effect::set_operation(d, e3, Some(operation(spop2)));
    card::register_effect(d, c, e3, false);
}
/// `s.spcost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return #pg<=1 and c:IsFaceup() and c:IsSetCard(SET_SUPER_QUANTUM)
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetAttribute(),pg)
    if pg.len() > 1 || !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_SUPER_QUANTUM as u16]) {
        return false;
    }
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp, c, att, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter2(c,e,tp,mc,att,pg)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, att: u32, pg: &CardSet) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_SUPER_QUANTAL_MECH_BEAST) and c:IsAttribute(att) and mc:IsCanBeXyzMaterial(c,tp)
    //     and (#pg<=0 or pg:IsContains(mc)) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_SUPER_QUANTAL_MECH_BEAST as u16])
        && card::is_attribute(d, c, att)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && (pg.len() <= 0 || group::is_contains(pg, mc))
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.spfilter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && spfilter1(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.spfilter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, a.tp, Some(&|d, c| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = match duel::first_target(d) {
        Some(tc) => tc,
        None => return,
    };
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) || card::is_controler(d, tc, 1 - a.tp) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(tc)), a.tp, None, None, REASON_XYZ);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetAttribute(),pg)
    let att = card::get_attribute(d, tc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter2(d, x, a.e, a.tp, tc, att, &pg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `s.spfilter3(c,e)`
fn spfilter3(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsSetCard(SET_SUPER_QUANTAL_MECH_BEAST)
    //     and c:IsType(TYPE_XYZ) and c:IsCanBeEffectTarget(e)
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_set_card(d, c, &[SET_SUPER_QUANTAL_MECH_BEAST as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.spfilter4(c,e,tp,rp)`
fn spfilter4(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: Option<u8>) -> bool {
    // return c:IsCode(84025439) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0
    card::is_code(d, c, &[84025439])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, rp, NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter3,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter3(d, c, a.e)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>2
    //     and Duel.IsExistingMatchingCard(s.spfilter4,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 2
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter4(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // local eg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_XMATERIAL)
    let eg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_XMATERIAL as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter4,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let rp = a.rp;
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter4(d, c, a.e, a.tp, Some(rp))), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=sg:GetFirst()
    let sc = group::get_first(d, &sg);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            // local g=Duel.GetTargetCards(e):Match(aux.NOT(Card.IsImmuneToEffect),nil,e)
            let mut g = duel::get_target_cards(d, a.e).nil();
            group::match_(d, &mut g, &|d, c| !card::is_immune_to_effect(d, c, a.e), NO_EX);
            // Duel.Overlay(sc,g)
            duel::overlay(d, sc, g, false);
        }
    }
}
