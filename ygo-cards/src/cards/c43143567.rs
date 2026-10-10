//! Fallen Angel of the Golden Land
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 43143567;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GOLDEN_LORD, ID], material_setcode: &[SET_ELDLICH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Eldlich the Golden Lord" + 1 Zombie monster
    // Fusion.AddProcMix(c,true,true,CARD_GOLDEN_LORD,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ZOMBIE))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_GOLDEN_LORD),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_ZOMBIE, Some(fc), st, Some(tp)))),
        ],
    );
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // You can only Fusion Summon or Special Summon by its alternate procedure "Angelcaido of the Golden Land" once per turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can Special Summon this card by Tributing 1 LIGHT Zombie monster while you have "Eldlich the Golden Lord" in your field or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e1, LOCATION_EXTRA);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Eldlich" monster from your Deck, Extra Deck, or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_ONFIELD)
    })));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regcon(e)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1)
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // --Prevent another Fusion Summon or Special Summon by its alternate procedure of "Angelcaido of the Golden Land" that turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return c:IsCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1) end)
    effect::set_target(d, e1, Some(lua_fn(regtarget)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,c,sump,sumtype)` of `s.regop`
fn regtarget(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    // return c:IsCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1)
    card::is_code(d, c, &[ID])
        && ((sumtype & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64 || (sumtype & (SUMMON_TYPE_SPECIAL as u64 + 1)) == SUMMON_TYPE_SPECIAL as u64 + 1)
}
/// `s.selfspcostfilter(c,tp,sc)`
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_ZOMBIE) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_ZOMBIE)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if c:IsFaceup() then return false end
    if card::is_faceup(d, c) {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GOLDEN_LORD),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    //     and Duel.CheckReleaseGroup(tp,s.selfspcostfilter,1,false,1,true,c,tp,nil,nil,nil,tp,c)
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_GOLDEN_LORD])), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::check_release_group(
            d,
            tp,
            Some(&|d, x| selfspcostfilter(d, x, tp, c)),
            1,
            duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
            NO_EX,
            false,
        )
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c)` of `e1:SetTarget`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.selfspcostfilter,1,1,false,true,true,c,tp,nil,false,nil,tp,c)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| selfspcostfilter(d, x, tp, c)),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    );
    // if not g then return false end
    let Some(g) = g else {
        return false;
    };
    // g:KeepAlive()
    // e:SetLabelObject(g)
    effect::set_label_object(d, e, Value::Group(g));
    // return true
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of `e1:SetOperation`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
/// `s.spfilter(c,e,tp,ft)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // if not (c:IsSetCard(SET_ELDLICH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_ELDLICH as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return ft>0
        ft > 0
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, ft)), tp, LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,e,tp,ft)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, ft))),
        tp,
        LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
