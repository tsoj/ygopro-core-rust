//! Don Thousand's Throne
use super::prelude::*;

pub const ID: u32 = 93238626;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_NUMBER_C], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.reccon)
    effect::set_condition(d, e2, Some(condition(reccon)));
    // e2:SetTarget(s.rectg)
    effect::set_target(d, e2, Some(cost_target(rectg)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DAMAGE)
        effect::set_code(d, ge1, EVENT_BATTLE_DAMAGE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // Duel.RegisterFlagEffect(ep,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.ep, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.reccon(e,tp,...)`: `Duel.IsTurnPlayer(1-tp)`
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.rectg(e,tp,...,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetFlagEffect(tp,id)
    let ct = duel::get_flag_effect(d, tp, ID);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,ct*500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, (ct * 500) as i32);
    true
}
/// `s.recop(e,tp,...)`
fn recop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // local ct=Duel.GetFlagEffect(tp,id)
    let ct = duel::get_flag_effect(d, tp, ID);
    // Duel.Recover(p,ct*500,REASON_EFFECT)
    duel::recover(d, p, (ct * 500) as i64, REASON_EFFECT, false, None);
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // return at:IsFaceup() and at:IsControler(tp) and at:IsSetCard(SET_NUMBER) and not at:IsSetCard(SET_NUMBER_C)
    card::is_faceup(d, at)
        && card::is_controler(d, at, a.tp)
        && card::is_set_card(d, at, &[SET_NUMBER as u16])
        && !card::is_set_card(d, at, &[SET_NUMBER_C as u16])
}
/// `s.filter(c,e,tp,mc,no,pg)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, no: Option<u32>, pg: &CardSet) -> bool {
    // return c.xyz_number==no and c:IsSetCard(SET_NUMBER_C) and mc:IsCanBeXyzMaterial(c,tp)
    //   and (#pg<=0 or pg:IsContains(mc)) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    crate::card_info(d, c).xyz_number == no
        && card::is_set_card(d, c, &[SET_NUMBER_C as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && (pg.is_empty() || group::is_contains(pg, mc))
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local at=Duel.GetAttackTarget()
        let at = duel::get_attack_target(d).nil();
        // local m=at:GetMetatable(true)
        let m = crate::script_info(card::get_code(d, at).first().copied().nil());
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(at),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(at)), tp, None, None, REASON_XYZ as u32);
        // return #pg<=1 and m and m.xyz_number and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,at,m.xyz_number,pg)
        return pg.len() <= 1
            && m.xyz_number.is_some()
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, at, m.xyz_number, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if not Duel.NegateAttack() then return end
    if !duel::negate_attack(d) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 0 {
        return;
    }
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d).nil();
    // local m=tc:GetMetatable(true)
    let m = crate::script_info(card::get_code(d, tc).first().copied().nil());
    // if not tc or tc:IsFacedown() or not tc:IsRelateToBattle() or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or not m then return end
    // (a card without a script is `not m` in Lua: every card here has one)
    if card::is_facedown(d, tc) || !card::is_relate_to_battle(d, tc) || card::is_controler(d, tc, 1 - tp) || card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ as u32);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,m.xyz_number,pg)
    let no = m.xyz_number;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, tc, no, &pg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
