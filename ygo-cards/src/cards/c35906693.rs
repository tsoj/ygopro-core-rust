//! Shining Draw
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 35906693;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA, SET_ZW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // To activate this card, you must draw it for your normal draw in your Draw Phase, reveal it, and keep it revealed until the Main Phase 1
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EVENT_DRAW)
    effect::set_code(d, e0, EVENT_DRAW);
    // e0:SetCondition(function(e) return Duel.IsPhase(PHASE_DRAW) and e:GetHandler():IsReason(REASON_RULE) end)
    effect::set_condition(d, e0, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::is_phase(d, PHASE_DRAW) && card::is_reason(d, h, REASON_RULE)
    })));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function() return Duel.IsPhase(PHASE_MAIN1) end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_phase(d, PHASE_MAIN1))));
    // e1:SetCost(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return e:GetHandler():HasFlagEffect(id) end end)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1,EFFECT_FLAG_CLIENT_HINT,1,0,66)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1, EFFECT_FLAG_CLIENT_HINT, 1, 0, 66);
        // Reveal it, and keep it revealed until the Main Phase 1
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_PUBLIC)
        effect::set_code(d, e1, EFFECT_PUBLIC);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.tgfilter(c,e,tp,eq_chk)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eq_chk: bool) -> bool {
    // return c:IsSetCard(SET_UTOPIA) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    //     and (eq_chk or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c))
    card::is_set_card(d, c, &[SET_UTOPIA as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && (eq_chk || duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX))
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return c:IsSetCard(SET_UTOPIA) and not c:IsCode(mc:GetCode()) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    let mc_codes = card::get_code(d, mc);
    card::is_set_card(d, c, &[SET_UTOPIA as u16])
        && !card::is_code(d, c, &mc_codes)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ZW) and c:IsMonster() and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_ZW as u16])
        && card::is_monster(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local b=chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE)
        let b = card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE);
        // (ygoai override lua_fixes/c35906693.lua: the original calls the undefined s.utopiafilter and
        // s.filter2; the fix checks s.tgfilter for the chosen effect, 1 equip, 2 Xyz Summon)
        // if e:GetLabel()==1 then return b and s.tgfilter(chkc,e,tp,true)
        // else return b and s.tgfilter(chkc,e,tp,false) end
        let eq = effect::get_label(d, a.e) == 1;
        return b && tgfilter(d, chkc, a.e, a.tp, eq);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        ft -= 1;
    }
    // local eq_chk=ft>0 and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,tp)
    let tp = a.tp;
    let eq_chk = ft > 0 && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    // local b1=eq_chk and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,eq_chk)
    let b1 = eq_chk && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.e, tp, eq_chk)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,false)
    let mut b2 = duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.e, tp, false)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,eq_chk):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, a.e, tp, eq_chk)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // b2=Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,tc)
    b2 = duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let opt = effect::get_label(d, a.e);
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if opt == 1 {
        // Equip any number of "ZW -" monsters with different names from your Deck/Extra Deck to it
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if ft==0 then return end
        if ft == 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // if Duel.Equip(tp,sc,tc,true,true) then
            if duel::equip(d, tp, sc, tc, true, true) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetValue(function(e,c) return c==tc end)
                effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, x| x == tc));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
    } else if opt == 2 {
        // Special Summon from your Extra Deck, 1 "Utopia" Xyz Monster with a different name from that target, by using that target as material
        // if tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) then return end
        if card::is_controler(d, tc, 1 - tp) || card::is_immune_to_effect(d, tc, a.e) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sel) {
            // sc:SetMaterial(tc)
            card::set_material(d, sc, CardSet::from_card(tc));
            // Duel.Overlay(sc,tc)
            duel::overlay(d, sc, tc, false);
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)==0 then return end
            if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
                return;
            }
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
/// `s.cost` (`e:GetHandler():HasFlagEffect(id)` for the check)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():HasFlagEffect(id) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::has_flag_effect(d, h, ID, None);
    }
    true
}
