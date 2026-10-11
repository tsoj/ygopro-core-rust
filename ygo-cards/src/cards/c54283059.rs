//! Parallel World Fusion
use super::prelude::*;

pub const ID: u32 = 54283059;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_ELEMENTAL_HERO),aux.FALSE,s.fextra,Fusion.ShuffleMaterial)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]))),
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::shuffle_material()),
            ..Default::default()
        },
    );
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup,Card.IsAbleToDeck),tp,LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
        tp,
        LOCATION_REMOVED,
        0,
        NO_EX,
    );
    (Some(g), None)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON]).first() == Some(&0);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(a.e));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
// function s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetLabelObject()~=se
    let e = a.e(0);
    let label = effect::get_label_object(d, e).as_effect();
    let se = a.v(6).as_effect();
    label != se
}
