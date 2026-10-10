//! Dimension Fusion Destruction
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 89190953;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHANTASM], listed_names: &[6007213, 32491822, 69890967], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // Fusion.RegisterSummonEff{handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_PHANTASM),extrafil=s.fextra,
    //   chkf=FUSPROC_NOTFUSION|FUSPROC_LISTEDMATS,stage2=s.stage2,extraop=Fusion.BanishMaterial,extratg=s.extratg}
    fusion::register_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_PHANTASM as u16]))),
            extrafil: Some(Arc::new(fextra)),
            chkf: Some((FUSPROC_NOTFUSION | FUSPROC_LISTEDMATS) as u64),
            stage2: Some(Arc::new(stage2)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
}
/// `s.chlimit(e,ep,tp) return tp==ep end`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    a.p(2) == a.p(1)
}
/// `function(e,tp,mg) if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil) end end`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    (None, None)
}
/// `function(e,tc,tp,mg,chk) if chk==1 then ... tc:RegisterEffect(e1,true) end end`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    if chk == 1 {
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc.nil(), e1, true);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) ... end`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let (e, tp) = (a.e, a.tp);
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
        && effect::get_handler(d, e) == effect::get_owner(d, e)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[6007213, 32491822, 69890967])), tp, LOCATION_MZONE, 0, 1, NO_EX)
    {
        duel::set_chain_limit(d, lua_fn(chlimit));
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
