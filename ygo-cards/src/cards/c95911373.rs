//! Zoroa, the Magistus Conflagrant Calamity
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 95911373;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Equip 1 "Magistus" monster to itself
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(synchro_summoned_cond)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent the activation of effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    // e2:SetTargetRange(0,LOCATION_ALL)
    effect::set_target_range(d, e2, 0, LOCATION_ALL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetTarget(s.attg)
    effect::set_target(d, e2, Some(effect_card_fn(attg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 "Magistus" card and Special Summon itself
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn synchro_summoned_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_EXTRA,0,1,nil,SET_MAGISTUS) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_set_card(d, x, &[SET_MAGISTUS as u16])), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,Card.IsSetCard,tp,LOCATION_EXTRA,0,1,1,nil,SET_MAGISTUS):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_set_card(d, x, &[SET_MAGISTUS as u16])), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let ec = group::get_first(d, &sel);
    // if ec then
    if let Some(ec) = ec {
        // Duel.Equip(tp,ec,c,true)
        duel::equip(d, tp, ec, c, true, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.atfilter(c,ty)`
fn atfilter(d: &mut Duel, c: CardId, ty: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGISTUS) and c:IsOriginalType(TYPE_MONSTER) and c:IsOriginalType(ty)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MAGISTUS as u16])
        && card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_original_type(d, c, ty)
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local ty=c:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)
    let ty = card::get_type(d, c) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK);
    // if ty==0 then return false end
    if ty == 0 {
        return false;
    }
    // return Duel.IsExistingMatchingCard(s.atfilter,e:GetHandlerPlayer(),LOCATION_STZONE,0,1,nil,ty)
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&|d, x| atfilter(d, x, ty)), p, LOCATION_STZONE, 0, 1, NO_EX)
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGISTUS) and Duel.GetMZoneCount(tp,c)>0
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MAGISTUS as u16])
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return ft>-1 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return ft > -1
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
