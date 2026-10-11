//! Future Fusion Nova
use super::prelude::*;

pub const ID: u32 = 92058902;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: Fusion Summon 1 LIGHT Machine Fusion Monster from your Extra Deck, ...
    // local fusion_params={handler=c,fusfilter=function(c) return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) end,
    //   matfilter=aux.FALSE,extrafil=s.fextra,extratg=s.extratg,stage2=s.stage2}
    // local e1=Fusion.CreateSummonEff(fusion_params)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE))),
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When this card leaves the field, destroy that monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    // e2:SetOperation(function(e) e:SetLabel(e:GetHandler():IsDisabled() and 1 or 0) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let label = if card::is_disabled(d, h) { 1 } else { 0 };
        effect::set_label(d, a.e, &[label]);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetOperation(s.mondesop)
    effect::set_operation(d, e3, Some(operation(mondesop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // When that monster is destroyed, destroy this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.selfdescon)
    effect::set_condition(d, e4, Some(condition(selfdescon)));
    // e4:SetOperation(function(e) Duel.Destroy(e:GetHandler(),REASON_EFFECT) end)
    effect::set_operation(d, e4, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_EFFECT, None, None);
    })));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    // return sg:IsExists(Card.IsCode,1,nil,CARD_CYBER_DRAGON)
    group::is_exists(d, &sg, &|d, c| card::is_code(d, c, &[CARD_CYBER_DRAGON]), 1, NO_EX)
}
/// `s.fextra(e,tp,mg1)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg1: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil),s.fcheck
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.stage2(e,fc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, fc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if chk == 1 && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // c:SetCardTarget(fc)
        card::set_card_target(d, c, fc.nil());
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(function(e,c) return c:IsRaceExcept(RACE_MACHINE) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_MACHINE, None, 0, None))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local fid=fc:GetFieldID()
        let fid = card::get_field_id(d, fc.nil());
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
        aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
        // e2:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e2, LOCATION_MZONE, 0);
        // e2:SetTarget(function(e,c) return c:GetFieldID()~=fid end)
        effect::set_target(d, e2, Some(effect_card_fn(move |d, _e, c| card::get_field_id(d, c) != fid)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.mondesop(e,tp,eg,ep,ev,re,r,rp)`
fn mondesop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, label_obj) != 0 {
        return;
    }
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsLocation(LOCATION_MZONE) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.selfdescon(e,tp,eg,ep,ev,re,r,rp)`
fn selfdescon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    match card::get_first_card_target(d, h) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
        }
        None => false,
    }
}
