//! Number 4: Stealth Kragen
//! scripted by XyLeN
use super::prelude::*;

pub const ID: u32 = 67557908;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94942656], listed_series: &[SET_NUMBER as u32], xyz_number: Some(4), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // All monsters on the field becomes WATER
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetValue(ATTRIBUTE_WATER)
    effect::set_value(d, e1, ATTRIBUTE_WATER as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster and inflict damage equal to its ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some((TIMINGS_CHECK_MONSTER | TIMING_MAIN_END) as u32));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon "Kragen Spawn" up to the number of materials of this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Record of each materials before this card leaves the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetOperation(s.recordop)
    effect::set_operation(d, e4, Some(operation(recordop)));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.desfilter`: `aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER)`
fn water_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&water_faceup), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&water_faceup), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&water_faceup), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        // local atk=math.min(g:GetFirst():GetAttack()/2)
        let mut atk = card::get_attack(d, tc) as f64 / 2.0;
        // if atk<0 then atk=0 end
        if atk < 0.0 {
            atk = 0.0;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if Duel.Destroy(g:GetFirst(),REASON_EFFECT)>0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, lua_int(atk), REASON_EFFECT, false, None);
        }
    }
}
/// `s.recordop(e,tp,eg,ep,ev,re,r,rp)`
fn recordop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetOverlayGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, h);
    // e:GetLabelObject():SetLabelObject(g)
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, e3, Value::Group(g));
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsXyzSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_xyz_summoned(d, c)
}
/// `s.spfilter(c,e,tp,rp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // return c:IsCode(94942656) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[94942656])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if chk==0 then return #g>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp) end
    if a.chk == Some(0) {
        let (e, tp, rp) = (a.e, a.tp, a.rp);
        return !g.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    // local ct=#e:GetLabelObject()
    let g_label = effect::get_label_object(d, e).as_group().cloned().nil();
    let mut ct = g_label.len() as i64;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // ct=math.min(ct,aux.CheckSummonGate(tp) or ct)
    let gate = aux::check_summon_gate_limit(d, tp).unwrap_or(ct);
    ct = ct.min(gate);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,ct,nil,e,tp,rp)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local count=Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        let count = duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // if count~=0 then
        if count != 0 {
            // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAttribute),tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_WATER)
            let mut sg = duel::get_matching_group(
                d,
                Some(&aux::necro_valley_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER))),
                tp,
                LOCATION_GRAVE,
                0,
                NO_EX,
            );
            // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
                // sg=sg:Select(tp,1,count,nil)
                sg = group::select(d, &sg, tp, 1, count as u16, false, NO_EX).nil();
                // for oc in aux.Next(sg) do
                for oc in group::iter(d, &sg) {
                    // local tc=g:FilterSelect(tp,Card.IsLocation,1,1,nil,LOCATION_MZONE):GetFirst()
                    let sel = group::filter_select(d, &g, tp, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, 1, false, NO_EX).nil();
                    let Some(tc) = group::get_first(d, &sel) else {
                        // if not tc then break end
                        break;
                    };
                    // Duel.Overlay(tc,oc)
                    duel::overlay(d, tc, oc, false);
                    // g:RemoveCard(tc)
                    group::remove_card(&mut g, &CardSet::from_card(tc));
                }
            }
        }
    }
}
