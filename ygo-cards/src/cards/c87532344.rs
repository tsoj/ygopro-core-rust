//! Eternal Favorite
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 87532344;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_YUBEL], listed_series: &[SET_YUBEL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUBEL) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_YUBEL as u16]) && card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.fcheck(tp,sg,fc)`: `return sg:IsExists(Card.IsSetCard,1,nil,SET_YUBEL)`
fn fcheck() -> LuaFn {
    lua_fn(|d, a| {
        let sg = a.g(1);
        group::is_exists(d, &sg, &|d, c| card::is_set_card(d, c, &[SET_YUBEL as u16]), 1, NO_EX)
    })
}
/// `s.fextra(e,tp,mg)`: `return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup),tp,0,LOCATION_ONFIELD,nil),s.fcheck`
fn fustg_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // Fusion.SummonEffTG(nil,Fusion.OnFieldMat,s.fextra)
        matfilter: Some(fusion::on_field_mat()),
        extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| {
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
            (Some(g), Some(fcheck()))
        })),
        ..Default::default()
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local fustg=Fusion.SummonEffTG(nil,Fusion.OnFieldMat,s.fextra)
    // Special Summon 1 of your "Yubel" monsters that is banished or in your GY
    // local b1=not Duel.HasFlagEffect(tp,id) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    // Fusion Summon using monsters from either field as material, including a "Yubel" monster
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_YUBEL),tp,LOCATION_ONFIELD,0,1,nil)
    //     and c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil)
    //     and fustg(e,tp,eg,ep,ev,re,r,rp,0)
    let mut call0 = a.clone();
    call0.chk = Some(0);
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, cc| card::is_code(d, cc, &[CARD_YUBEL]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && card::is_able_to_grave_as_cost(d, c)
        && card::is_status(d, c, STATUS_EFFECT_ENABLED)
        && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_discardable(d, cc, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && fusion::summon_eff_tg_call(d, &fustg_params(), &call0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
        duel::discard_hand(d, tp, Some(&|d, cc| card::is_discardable(d, cc, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
        // Duel.SendtoGrave(c,REASON_COST)
        duel::send_to_grave(d, c, REASON_COST, None, None);
        // fustg(e,tp,eg,ep,ev,re,r,rp,1)
        let mut call1 = a.clone();
        call1.chk = Some(1);
        fusion::summon_eff_tg_call(d, &fustg_params(), &call1);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // Special Summon 1 of your "Yubel" monsters that is banished or in your GY
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, cc| spfilter(d, cc, e, tp))),
            tp,
            LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if let Some(sc) = sc {
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
                card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_CHAIN_END)
                effect::set_code(d, e1, EVENT_CHAIN_END);
                // e1:SetLabelObject(sc)
                effect::set_label_object(d, e1, Value::Card(sc));
                // e1:SetOperation(s.limop)
                effect::set_operation(d, e1, Some(lua_fn(limop)));
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else if op == 2 {
        // Fusion.SummonEffOP(nil,Fusion.OnFieldMat,s.fextra)(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fustg_params(), a);
    }
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e).as_card();
    // local ex,sg=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    // if ex and #sg==1 and sg:GetFirst()==sc and sc:HasFlagEffect(id) then
    if let Some(ev) = duel::check_event(d, EVENT_SPSUMMON_SUCCESS) {
        let sg = ev.event_cards.clone().nil();
        if sg.len() == 1 && group::get_first(d, &sg) == sc {
            if let Some(sc) = sc {
                if card::has_flag_effect(d, sc, ID, None) {
                    // Duel.SetChainLimitTillChainEnd(aux.FALSE)
                    duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, _a: Args| false));
                }
            }
        }
    }
    // e:Reset()
    effect::reset(d, e);
}
