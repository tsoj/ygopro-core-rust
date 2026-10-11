//! Performapal Odd-Eyes Dissolver
use super::prelude::*;

pub const ID: u32 = 46136942;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // fusion from the pendulum zone
    // local params = {aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON)}
    let params = || fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON))),
        ..Default::default()
    };
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg1 = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e1, Some(tg1));
    // e1:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op1 = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e1, Some(op1));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_START);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // fusion from the monster zone
    // local params2 = {nil,Fusion.CheckWithHandler(Fusion.OnFieldMat),s.fextra,nil,Fusion.ForcedHandler}
    let params2 = || fusion::SummonEff {
        matfilter: Some(fusion::check_with_handler(fusion::on_field_mat())),
        extrafil: Some(std::sync::Arc::new(fextra)),
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    };
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    let tg3 = fusion::summon_eff_tg(d, params2());
    effect::set_target(d, e3, Some(tg3));
    let op3 = fusion::summon_eff_op(d, params2());
    effect::set_operation(d, e3, Some(op3));
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local a=Duel.GetAttacker()
    // local d=a:GetBattleTarget()
    let mut at = duel::get_attacker(d).nil();
    let Some(mut dd) = card::get_battle_target(d, at) else {
        // if not d then return false end
        return false;
    };
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, at, 1 - tp) {
        std::mem::swap(&mut at, &mut dd);
    }
    // e:SetLabelObject(a)
    effect::set_label_object(d, e, Value::Card(at));
    // return a:IsControler(tp) and a:IsFaceup() and a:IsType(TYPE_PENDULUM) and a:GetControler()~=d:GetControler()
    card::is_controler(d, at, tp)
        && card::is_faceup(d, at)
        && card::is_type(d, at, TYPE_PENDULUM)
        && card::get_controler(d, at) != card::get_controler(d, dd)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local tc=e:GetLabelObject()
        let tc = effect::get_label_object(d, e).as_card().nil();
        // if not tc:IsRelateToBattle() then return end
        if !card::is_relate_to_battle(d, tc) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_PZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_PZONE, 0, NO_EX);
    (Some(g), None)
}
