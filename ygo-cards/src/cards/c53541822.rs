//! Ancient Gear Duel
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 53541822;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_GEAR_GOLEM], listed_series: &[], ..ScriptInfo::DEFAULT };

/// `params` of `Fusion.SummonEffTG` / `Fusion.SummonEffOP`
fn params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:ListsCode(CARD_ANCIENT_GEAR_GOLEM) end
        fusfilter: Some(Arc::new(|d, c, _tp| card::lists_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]))),
        // matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove)
        matfilter: Some(fusion::on_field_mat_with(Arc::new(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))))),
        // extrafil=s.fmatextra
        extrafil: Some(Arc::new(|d, e, tp, mg| fmatextra(d, e, tp, mg))),
        // extratg=s.extratarget
        extratg: Some(cost_target(extratarget)),
        // extraop=Fusion.BanishMaterial
        extraop: Some(fusion::banish_material()),
        // stage2=s.stage2
        stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
        ..Default::default()
    }
}

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
    // "Ancient Gear Golem" and monsters that mention it are unaffected by your opponent's activated monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(function(e,c) return c:IsCode(CARD_ANCIENT_GEAR_GOLEM) or c:ListsCode(CARD_ANCIENT_GEAR_GOLEM) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) || card::lists_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]))));
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(immval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Fusion Summon 1 Fusion Monster that mentions "Ancient Gear Golem"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0)));
    // e3:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e3, Some(tg));
    // e3:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e3, Some(op));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() and te:IsMonsterEffect()
    let te = a.e(1);
    effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, a.e(0)) && effect::is_activated(d, te) && effect::is_monster_effect(d, te)
}
/// `s.aggfilter(c,tp)`
fn aggfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_ANCIENT_GEAR_GOLEM) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) && card::is_location(d, c, LOCATION_MZONE) && card::get_controler(d, c) == tp
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(s.aggfilter,1,nil,tp)
    let tp = a.p(0);
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| aggfilter(d, c, tp), 1, NO_EX)
}
/// `s.fmatextra(e,tp,mg)`
fn fmatextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil),s.fcheck
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), Some(lua_fn(fcheck)));
    }
    // return nil,s.fcheck
    (None, Some(lua_fn(fcheck)))
}
/// `s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // Can make up to 3 attacks during each Battle Phase
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(2)
        effect::set_value(d, e1, 2);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
