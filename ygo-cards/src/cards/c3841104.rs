//! Destratification Dino Stratiae
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 3841104;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters, including a Dinosaur monster
    // Link.AddProcedure(c,nil,2,2,s.linkmatcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(linkmatcheck)), None, None);
    // Keep track of the total original ATK of the Dinosaur monsters used as its material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(function(e,c) ... end)
    effect::set_value_fn(d, e0, lua_fn(e0_value));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card is Link Summoned: You can make this card gain ATK equal to half the total original ATK of the Dinosaur monsters used as its material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() and e0:GetLabel()>0 end)
    effect::set_condition(d, e1, Some(condition(move |d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h) && effect::get_label(d, e0) > 0
    })));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e1, Some(cost_target(move |d, a| e1_target(d, a, e0))));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can Fusion Summon 1 Dinosaur Fusion Monster from your Extra Deck, by banishing its materials from your field and/or GY, also you cannot Special Summon from the Extra Deck for the rest of this turn, except Dinosaur monsters. You can only use this effect of "Destratification Dino Stratiae" once per turn
    // local fusion_params={...}
    let params = fusion::SummonEff {
        // fusfilter=function(c) return c:IsRace(RACE_DINOSAUR) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DINOSAUR))),
        // matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove)
        matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| {
            card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))
        }))),
        // extrafil=function(e,tp,mg) ... end
        extrafil: Some(Arc::new(extrafil)),
        // extraop=Fusion.BanishMaterial
        extraop: Some(fusion::banish_material()),
        // extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end
        extratg: Some(cost_target(extratg)),
        // stage2=function(e,fc,tp,mg,chk) ... end
        stage2: Some(Arc::new(stage2)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.linkmatcheck(g,linkc,sumtype,tp)`
fn linkmatcheck(d: &mut Duel, g: &CardSet, linkc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_DINOSAUR,linkc,sumtype,tp)
    let _ = (linkc, sumtype, tp);
    group::is_exists(d, g, &|d, c| card::is_race(d, c, RACE_DINOSAUR), 1, NO_EX)
}
/// `function(e,c)` of `EFFECT_MATERIAL_CHECK`: `e:SetLabel(mg:GetSum(Card.GetBaseAttack)//2)`
fn e0_value(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let c = a.c(1);
    // local mg=c:GetMaterial():Match(Card.IsRace,nil,RACE_DINOSAUR,c,SUMMON_TYPE_LINK,e:GetHandlerPlayer())
    let mut mg = card::get_material(d, c);
    group::match_(d, &mut mg, &|d, x| card::is_race(d, x, RACE_DINOSAUR), NO_EX);
    // e:SetLabel(mg:GetSum(Card.GetBaseAttack)//2)
    let sum = group::get_sum(d, &mg, &|d, x| card::get_base_attack(d, x) as i64);
    effect::set_label(d, e, &[sum.div_euclid(2)]);
    // (no return value: nil, read by the core as 0)
    0
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e1: `e0` is the Material Check effect
fn e1_target(d: &mut Duel, a: &EffArgs, e0: EffectId) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local atk=e0:GetLabel()
    let atk = effect::get_label(d, e0);
    // e:GetChainData().atk=atk
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(atk));
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,atk)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, h.map(CardSet::from_card), 1, a.tp, atk as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e1
fn e1_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // c:UpdateAttack(e:GetChainData().atk)
        let atk = chain::effect_data(d, a.e, None, chain::field::CARD).int();
        card::update_attack(d, c, atk, None, None, None);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of `fusion_params.extratg`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `function(e,tp,mg)` of `fusion_params.extrafil`
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let f = fusion::is_monster_filter(fusion::fus_filter(|d, c, _tp| card::is_able_to_remove(d, c, None, None, None)));
        return (Some(duel::get_matching_group(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_GRAVE, 0, NO_EX)), None);
    }
    // return nil
    (None, None)
}
/// `function(e,fc,tp,mg,chk)` of `fusion_params.stage2`
fn stage2(d: &mut Duel, e: EffectId, _fc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk~=2 then return end
    if chk != 2 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Dinosaur monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and c:IsRaceExcept(RACE_DINOSAUR) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && card::is_race_except(d, c, RACE_DINOSAUR, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(c) return not c:IsOriginalRace(RACE_DINOSAUR) end)
    aux::add_temp_lizard_check(d, c, tp, Some(lua_fn(|d, a| !card::is_original_race(d, a.c(0), RACE_DINOSAUR))), None, None, None, None);
}
