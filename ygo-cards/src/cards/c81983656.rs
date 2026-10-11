//! BF T－漆黒のホーク・ジョー
//! Blackwing Tamer - Obsidian Hawk Joe
use super::prelude::*;

pub const ID: u32 = 81983656;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_BLACKWING),1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_BLACKWING),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_BLACKWING as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_BLACKWING as u16], sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.cbcon)
    effect::set_condition(d, e2, Some(condition(cbcon)));
    // e2:SetTarget(s.cbtg)
    effect::set_target(d, e2, Some(cost_target(cbtg)));
    // e2:SetOperation(s.cbop)
    effect::set_operation(d, e2, Some(operation(cbop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // change effect target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.cecon)
    effect::set_condition(d, e3, Some(condition(cecon)));
    // e3:SetTarget(s.cetg)
    effect::set_target(d, e3, Some(cost_target(cetg)));
    // e3:SetOperation(s.ceop)
    effect::set_operation(d, e3, Some(operation(ceop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsRace(RACE_WINGEDBEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_WINGEDBEAST) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r~=REASON_REPLACE
    a.r != REASON_REPLACE
}
/// `s.cbfilter(c,at)`
fn cbfilter(d: &mut Duel, c: CardId, at: &CardSet) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BLACKWING) and at:IsContains(c)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16]) && group::is_contains(at, c)
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker():GetAttackableTarget()
    let attacker = duel::get_attacker(d).nil();
    let (at, _) = card::get_attackable_target(d, attacker);
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.cbfilter(chkc,at) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && cbfilter(d, chkc, &at);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cbfilter,tp,LOCATION_MZONE,0,1,e:GetHandler(),at) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cbfilter(d, c, &at)), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.cbfilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler(),at)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| cbfilter(d, c, &at)), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, duel::get_attacker(d).nil(), a.e) {
            // Duel.ChangeAttackTarget(tc)
            duel::change_attack_target(d, Some(tc), false);
        }
    }
}
/// `s.cecon(e,tp,eg,ep,ev,re,r,rp)`
fn cecon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e==re or rp==tp or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.re == Some(a.e) || a.rp == a.tp || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned();
    // return g and #g==1 and g:GetFirst()==e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    match g {
        Some(g) if g.len() == 1 => group::get_first(d, &g) == Some(h),
        _ => false,
    }
}
/// `s.cefilter(c,ct)`
fn cefilter(d: &mut Duel, c: CardId, ct: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BLACKWING) and Duel.CheckChainTarget(ct,c)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16]) && duel::check_chain_target(d, ct as u8, c)
}
/// `s.cetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cetg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.cefilter(chkc,ev) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && cefilter(d, chkc, a.ev);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cefilter,tp,LOCATION_MZONE,0,1,e:GetHandler(),ev) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cefilter(d, c, a.ev)), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.cefilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler(),ev)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| cefilter(d, c, a.ev)), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    true
}
/// `s.ceop(e,tp,eg,ep,ev,re,r,rp)`
fn ceop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangeTargetCard(ev,Group.FromCards(tc))
            duel::change_target_card(d, a.ev as u8, &CardSet::from_card(tc));
        }
    }
}
