//! Miracle Synchro Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 36484016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,function(c) return c.miracle_synchro_fusion end,Fusion.OnFieldMat(Card.IsAbleToRemove),
    //     s.fextra,Fusion.BanishMaterial,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| crate::card_info(d, c).miracle_synchro_fusion)),
            matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| {
                card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))
            }))),
            extrafil: Some(Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fcheck(tp,sg,fc)`: `return sg:IsExists(aux.FilterBoolFunction(Card.IsType,TYPE_SYNCHRO,fc,SUMMON_TYPE_FUSION,tp),1,nil)`
fn fcheck_fn() -> LuaFn {
    lua_fn(|d, a: Args| {
        let tp = a.p(0);
        let sg = a.g(1);
        let fc = a.card(2);
        group::is_exists(
            d,
            &sg,
            &|d, c| card::is_type_ctx(d, c, TYPE_SYNCHRO, fc, SUMMON_TYPE_FUSION as u64, Some(tp)),
            1,
            NO_EX,
        )
    })
}
/// `s.fextra(e,tp,mg)`: the Graveyard monsters that can be banished (unless Spirit Elimination applies)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil),s.fcheck
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        );
        return (Some(g), Some(fcheck_fn()));
    }
    // return nil,s.fcheck
    (None, Some(fcheck_fn()))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&(REASON_DESTROY|REASON_EFFECT))==(REASON_DESTROY|REASON_EFFECT) and rp~=tp and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    let both = REASON_DESTROY | REASON_EFFECT;
    (a.r & both) == both
        && a.rp != a.tp
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN as u32)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
