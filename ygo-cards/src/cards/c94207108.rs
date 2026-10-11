//! Marincess Wonder Heart
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 94207108;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Special Summon equipped
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(condition(spcon1)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,Card.EquipByEffectAndLimitRegister,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_fn), e1, None, None, None);
    // Special Summon from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(ec,c,tp)` of `aux.AddEREquipLimit`: `s.eqval`
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsSetCard(SET_MARINCESS)
    let ec = a.c(0);
    card::is_set_card(d, ec, &[SET_MARINCESS as u16])
}
/// The equip operation: `Card.EquipByEffectAndLimitRegister` called with `(c,e,tp,tc)`
fn equipop_fn(d: &mut Duel, a: Args) {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None);
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    card::get_battle_target(d, h).is_some()
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:GetEquipGroup():IsExists(s.spfilter1,1,nil,e,tp) and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let (count, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        if count <= 0 {
            return false;
        }
        let eqg = card::get_equip_group(d, c);
        if !group::is_exists(d, &eqg, &|d, x| spfilter1(d, x, a.e, a.tp), 1, NO_EX) {
            return false;
        }
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_SZONE as i32);
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let (count, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
    if count < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=c:GetEquipGroup():FilterSelect(tp,s.spfilter1,1,1,nil,e,tp):GetFirst()
    let eqg = card::get_equip_group(d, c);
    let sel = group::filter_select(d, &eqg, a.tp, &|d, x| spfilter1(d, x, a.e, a.tp), 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c) as i64;
            // tc:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD);
            // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
            effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
            // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e2:SetTargetRange(1,0)
            effect::set_target_range(d, e2, 1, 0);
            // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e3:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
            // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e3:SetCountLimit(1)
            effect::set_count_limit(d, e3, 1, 0, 0, 0);
            // e3:SetLabel(fid)
            effect::set_label(d, e3, &[fid]);
            // e3:SetLabelObject(tc)
            effect::set_label_object(d, e3, Value::Card(tc));
            // e3:SetCondition(s.eqcon)
            effect::set_condition(d, e3, Some(condition(eqcon)));
            // e3:SetOperation(s.eqop)
            effect::set_operation(d, e3, Some(operation(eqop)));
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, a.tp);
            // c:CreateEffectRelation(e3)
            card::create_effect_relation(d, c, e3);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if not tc or tc:GetFlagEffectLabel(id+1)~=e:GetLabel() then
    let label = effect::get_label(d, a.e);
    let matches = match tc {
        Some(tc) => card::get_flag_effect_label(d, tc, ID + 1).first().copied() == Some(label),
        None => false,
    };
    if !matches {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,PLAYER_ALL,id)
    duel::hint(d, HINT_CARD as u8, PLAYER_ALL as u8, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    let (count, _) = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None);
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && count > 0 {
        // Duel.Equip(tp,tc,c,true)
        duel::equip(d, a.tp, tc.nil(), c, true, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(|d, e, x| effect::get_label_object(d, e).as_card() == Some(x)));
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    } else {
        // Duel.SendtoGrave(tc,REASON_RULE)
        duel::send_to_grave(d, tc.nil(), REASON_RULE, None, None);
    }
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_BATTLE) or (rp==1-tp and c:IsReason(REASON_EFFECT) and c:GetPreviousControler()==tp)
    card::is_reason(d, c, REASON_BATTLE) || (a.rp == 1 - a.tp && card::is_reason(d, c, REASON_EFFECT) && card::get_previous_controler(d, c) == a.tp)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsLinkBelow(3) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MARINCESS as u16])
        && card::is_link_below(d, c, 3)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (count, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        return count > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let (e, tp) = (a.e, a.tp);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(move |d, x| spfilter2(d, x, e, tp))),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
