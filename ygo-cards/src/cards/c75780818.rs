//! Dyna Tank
use super::prelude::*;

pub const ID: u32 = 75780818;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // card::enable_revive_limit
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Machine monster + 1 Dinosaur monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),aux.FilterBoolFunctionEx(Card.IsRace,RACE_DINOSAUR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DINOSAUR, Some(fc), st, Some(tp)))),
        ],
    );
    // Gain ATK
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.atkval)
    effect::set_value_fn(d, e0, effect_card_value(atkval));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Replace target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tarcon)
    effect::set_condition(d, e2, Some(condition(tarcon)));
    // e2:SetTarget(s.tartg)
    effect::set_target(d, e2, Some(cost_target(tartg)));
    // e2:SetOperation(s.tarop)
    effect::set_operation(d, e2, Some(operation(tarop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Dinosaur monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial():Filter(Card.IsRace,nil,RACE_DINOSAUR)
    let mats = card::get_material(d, c);
    let g = group::filter(d, &mats, &|d, cc| card::is_race(d, cc, RACE_DINOSAUR), NO_EX);
    // e:SetLabel(#g==1 and g:GetFirst():GetBaseAttack() or 0)
    let v = if g.len() == 1 {
        let first = group::get_first(d, &g).nil();
        card::get_base_attack(d, first) as i64
    } else {
        0
    };
    effect::set_label(d, e, &[v]);
    0
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local atk=e:GetLabelObject():GetLabel()
    let e0 = effect::get_label_object(d, a.e).as_effect().nil();
    let atk = effect::get_label(d, e0);
    // if atk>0 then
    if atk > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Increase ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// function s.tarcon(e,tp,eg,ep,ev,re,r,rp)
fn tarcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e==re or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if Some(a.e) == a.re {
        return false;
    }
    if !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // return #g==1 and g:GetFirst()==e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    g.len() == 1 && group::get_first(d, &g) == Some(h)
}
// function s.tarfilter(c,ev)
fn tarfilter(d: &mut Duel, c: CardId, ev: u32) -> bool {
    // return Duel.CheckChainTarget(ev,c)
    duel::check_chain_target(d, ev as u8, c)
}
// function s.tartg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tartg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, ev) = (a.tp, a.ev);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return c~=chkc and chkc:IsLocation(LOCATION_ONFIELD) and s.tarfilter(chkc,ev) end
    if let Some(chkc) = a.chkc() {
        return c != chkc && card::is_location(d, chkc, LOCATION_ONFIELD) && tarfilter(d, chkc, ev);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tarfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c,ev) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| tarfilter(d, cc, ev)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tarfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c,ev)
    let _ = duel::select_target(d, tp, Some(&|d, cc| tarfilter(d, cc, ev)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    true
}
// function s.tarop(e,tp,eg,ep,ev,re,r,rp)
fn tarop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.ChangeTargetCard(ev,Group.FromCards(tc))
        duel::change_target_card(d, a.ev as u8, &CardSet::from_card(tc));
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousControler(tp) and rp==1-tp
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_controler(d, c, a.tp) && a.rp == 1 - a.tp
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DINOSAUR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
