//! HRUM - Ultimate Force
//! ＨＲＵＭ－アルティメット・フォース
//! Hyper-Rank-Up-Magic Ultimate Force
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 87818869;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_UTOPIC, SET_NUMBER, SET_NUMBER_99],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 Rank 9 or lower "Utopia" or "Utopic" Xyz Monster you control whose original Attribute is LIGHT; Special Summon 1 Rank 10 or higher "Number" Xyz Monster from your Extra Deck, by using that Xyz Monster you control (this is treated as an Xyz Summon, transfer its materials), and if you do, attach this card to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // A "Number 99" Xyz Monster that has this card as material gains this effect: ● Once per Chain, when your opponent activates a monster effect during the Battle Phase (Quick Effect): Detach all materials from this card, and if you do, destroy all monsters your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e2, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_QUICK_F);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.tgfilter(c,e,tp)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or pg:IsContains(c)) and c:IsRankBelow(9) and c:IsSetCard(SET_UTOPIC) and c:IsOriginalAttribute(ATTRIBUTE_LIGHT) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    (pg.is_empty() || group::is_contains(&pg, c))
        && card::is_rank_below(d, c, 9)
        && card::is_set_card(d, c, &[SET_UTOPIC as u16])
        && card::is_original_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x: CardId| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    let rum_limit = crate::card_info(d, c).rum_limit;
    if let Some(rum_limit) = rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRankAbove(10) and c:IsSetCard(SET_NUMBER) and mc:IsCanBeXyzMaterial(c,tp)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank_above(d, c, 10)
        && card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, e, tp);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&|d: &mut Duel, c: CardId| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, tp, Some(&|d: &mut Duel, c: CardId| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) && !card::is_immune_to_effect(d, tc, e) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
        // if not (#pg<=0 or pg:IsContains(c)) then return end
        // (`c` is not yet assigned in this function: the Lua global is nil, and IsContains(nil) is false)
        if !pg.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, c: CardId| spfilter(d, c, e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let Some(xyz) = group::get_first(d, &sel) else {
            return;
        };
        // xyz:SetMaterial(tc)
        card::set_material(d, xyz, CardSet::from_card(tc));
        // Duel.Overlay(xyz,tc)
        duel::overlay(d, xyz, tc, false);
        // if Duel.SpecialSummon(xyz,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, xyz, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // xyz:CompleteProcedure()
            card::complete_procedure(d, xyz);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // if c:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, c, e) {
                // c:CancelToGrave()
                card::cancel_to_grave(d, c, true);
                // Duel.Overlay(xyz,c)
                duel::overlay(d, xyz, c, false);
            }
        }
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and rp==1-tp and re:IsMonsterEffect() and e:GetHandler():IsSetCard(SET_NUMBER_99)
    if !duel::is_battle_phase(d, None) || a.rp != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let handler = effect::get_handler(d, a.e).nil();
    card::is_set_card(d, handler, &[SET_NUMBER_99 as u16])
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local og=c:GetOverlayGroup()
    let og = card::get_overlay_group(d, c);
    // if c:IsRelateToEffect(e) and #og>0 and Duel.SendtoGrave(og,REASON_EFFECT)==#og then
    if card::is_relate_to_effect(d, c, a.e) && !og.is_empty() && duel::send_to_grave(d, og.clone(), REASON_EFFECT, None, None) == og.len() as i32 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
