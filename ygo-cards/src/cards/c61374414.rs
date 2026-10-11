//! CXyz N.As.Ch. Knight
//! ＣＸ－Ｎ・Ａｓ・Ｃｈ Ｋｎｉｇｈｔ
use super::prelude::*;

pub const ID: u32 = 61374414;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[34876719], listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,6,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(d, c, None, Some(6), 3, Some(std::sync::Arc::new(ovfilter)), Some(aux::stringid(ID, 0)), None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Xyz Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,34876719)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[34876719])
}
/// `s.spfilter(c,e,tp,mc,pg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, pg: &CardSet) -> bool {
    // local no=c.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_NUMBER)
    //     and no and no>=101 and no<=107
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and (#pg<=0 or pg:IsContains(mc))
    //     and mc:IsCanBeXyzMaterial(c,tp)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    if !(card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_NUMBER as u16])) {
        return false;
    }
    let in_range = matches!(no, Some(no) if no >= 101 && no <= 107);
    if !in_range {
        return false;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && (pg.is_empty() || group::is_contains(pg, mc))
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,pg)
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, c, &pg)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and e:GetLabelObject():GetFlagEffect(id)>0
    if !duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, lo, ID) > 0
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(sc,REASON_EFFECT)
    duel::destroy(d, sc, REASON_EFFECT, None, None);
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, a.tp) && !card::is_immune_to_effect(d, c, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c,pg)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, c, &pg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // local sc=g:GetFirst()
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // sc:SetMaterial(c)
            card::set_material(d, sc, CardSet::from_card(c));
            // Duel.Overlay(sc,c)
            duel::overlay(d, sc, c, false);
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
                // Destroy it during your opponent's next End Phase
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
                effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetLabelObject(sc)
                effect::set_label_object(d, e1, Value::Card(sc));
                // e1:SetCondition(s.descon)
                effect::set_condition(d, e1, Some(condition(descon)));
                // e1:SetOperation(s.desop)
                effect::set_operation(d, e1, Some(operation(desop)));
                // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
                // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
                card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, a.tp);
            }
        }
    }
}
